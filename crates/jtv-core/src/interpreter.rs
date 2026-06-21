// Interpreter for JtV
use crate::ast::*;
use crate::coproc::CoprocNamespace;
use crate::error::{JtvError, Result};
use crate::number::Value;
use crate::reversible::RecordedOp;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

const MAX_ITERATIONS: usize = 1_000_000; // Safety limit for loops

/// Callable registered for a live coproc function.
/// When present, the interpreter dispatches directly instead of returning
/// `ExternCoprocNotYetLowered`.  Used in tests and embedding contexts.
pub type NativeImpl = Arc<dyn Fn(&[Value]) -> Result<Value> + Send + Sync>;

pub struct Interpreter {
    globals: HashMap<String, Value>,
    /// Functions stored by qualified name (e.g., "Math::add" or just "add")
    functions: HashMap<String, FunctionDecl>,
    /// Extern coproc functions registered after PataCL resolution.
    /// Call-site evaluation returns ExternCoprocNotYetLowered (per ADR-0005).
    coproc_ns: CoprocNamespace,
    /// Native (Rust) implementations registered for lowered coproc functions.
    /// Keyed by unqualified function name, same as `coproc_ns`.
    native_impls: HashMap<String, NativeImpl>,
    /// v2 reversal token store: token_id → operation log.
    /// Tokens are linear; each is removed on `reverse tok` or `abandon tok`.
    token_store: HashMap<u64, Vec<RecordedOp>>,
    next_token_id: u64,
    /// Module definitions: module_name -> list of function names
    modules: HashMap<String, Vec<String>>,
    /// Imported modules (module_name -> optional alias)
    imports: HashMap<String, Option<String>>,
    call_stack: Vec<HashMap<String, Value>>,
    iteration_count: usize,
    trace_enabled: bool,
    trace: Vec<TraceEntry>,
    last_result: Option<Value>,
    /// Captured output from print statements (used by WASM and testing)
    output_buffer: Vec<String>,
    /// Whether to capture print output instead of printing to stdout
    capture_output: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceEntry {
    pub stmt_type: String,
    pub line: String,
    pub env: HashMap<String, String>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            globals: HashMap::new(),
            functions: HashMap::new(),
            coproc_ns: CoprocNamespace::default(),
            native_impls: HashMap::new(),
            token_store: HashMap::new(),
            next_token_id: 0,
            modules: HashMap::new(),
            imports: HashMap::new(),
            call_stack: vec![],
            iteration_count: 0,
            trace_enabled: false,
            trace: vec![],
            last_result: None,
            output_buffer: Vec::new(),
            capture_output: false,
        }
    }

    /// Register a PataCL-resolved coproc namespace so the interpreter
    /// can return the correct phase-boundary error at call sites.
    pub fn register_coproc_namespace(&mut self, ns: CoprocNamespace) {
        self.coproc_ns = ns;
    }

    /// Register a native (Rust) implementation for a lowered coproc function.
    ///
    /// When `name` is called and a native impl is registered, the interpreter
    /// dispatches to `f` instead of returning `ExternCoprocNotYetLowered`.
    /// This is the embedding hook used after Zig FFI lowering completes.
    pub fn register_coproc_impl(
        &mut self,
        name: impl Into<String>,
        f: impl Fn(&[Value]) -> Result<Value> + Send + Sync + 'static,
    ) {
        self.native_impls.insert(name.into(), Arc::new(f));
    }

    pub fn enable_trace(&mut self) {
        self.trace_enabled = true;
    }

    pub fn disable_trace(&mut self) {
        self.trace_enabled = false;
        self.trace.clear();
    }

    pub fn get_trace(&self) -> &[TraceEntry] {
        &self.trace
    }

    /// Enable output capture (print statements buffer instead of writing to stdout)
    pub fn enable_output_capture(&mut self) {
        self.capture_output = true;
    }

    /// Disable output capture (print statements go to stdout again)
    pub fn disable_output_capture(&mut self) {
        self.capture_output = false;
        self.output_buffer.clear();
    }

    /// Retrieve and clear the captured output buffer
    pub fn take_output(&mut self) -> Vec<String> {
        std::mem::take(&mut self.output_buffer)
    }

    /// Get captured output without clearing the buffer
    pub fn get_output(&self) -> &[String] {
        &self.output_buffer
    }

    /// Reset interpreter state completely (variables, functions, modules, trace, output)
    pub fn reset(&mut self) {
        self.globals.clear();
        self.functions.clear();
        self.coproc_ns = CoprocNamespace::default();
        self.native_impls.clear();
        self.token_store.clear();
        self.next_token_id = 0;
        self.modules.clear();
        self.imports.clear();
        self.call_stack.clear();
        self.iteration_count = 0;
        self.trace.clear();
        self.last_result = None;
        self.output_buffer.clear();
    }

    pub fn get_variables(&self) -> Vec<(String, Value)> {
        self.globals
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    pub fn get_last_result(&self) -> Option<&Value> {
        self.last_result.as_ref()
    }

    /// Return the names of all extern coproc functions registered in the
    /// current namespace, paired with their gate name.  Used by WASM bindings
    /// so the JS host knows which callback slots to fill.
    pub fn list_coproc_decls(&self) -> Vec<(String, String)> {
        self.coproc_ns
            .entries
            .iter()
            .map(|(fn_name, entry)| (entry.gate_name.clone(), fn_name.clone()))
            .collect()
    }

    fn add_trace(&mut self, stmt_type: &str, line: &str) {
        if self.trace_enabled {
            let env: HashMap<String, String> = self
                .globals
                .iter()
                .map(|(k, v)| (k.clone(), format!("{}", v)))
                .collect();

            self.trace.push(TraceEntry {
                stmt_type: stmt_type.to_string(),
                line: line.to_string(),
                env,
            });
        }
    }

    pub fn run(&mut self, program: &Program) -> Result<()> {
        self.last_result = None;
        self.trace.clear();
        for statement in &program.statements {
            self.eval_top_level(statement)?;
        }
        Ok(())
    }

    fn eval_top_level(&mut self, top_level: &TopLevel) -> Result<()> {
        self.eval_top_level_with_context(top_level, None)
    }

    fn eval_top_level_with_context(
        &mut self,
        top_level: &TopLevel,
        current_module: Option<&str>,
    ) -> Result<()> {
        match top_level {
            TopLevel::Module(module) => {
                // Register module and process its body
                let mut func_names = Vec::new();

                for stmt in &module.body {
                    if let TopLevel::Function(func) = stmt {
                        func_names.push(func.name.clone());
                    }
                    self.eval_top_level_with_context(stmt, Some(&module.name))?;
                }

                self.modules.insert(module.name.clone(), func_names);
                Ok(())
            }
            TopLevel::Import(import) => {
                // Register import
                let module_name = import.path.join("::");
                self.imports
                    .insert(module_name.clone(), import.alias.clone());

                // If module is already defined, make its functions available
                if let Some(func_names) = self.modules.get(&module_name).cloned() {
                    for func_name in func_names {
                        let qualified = format!("{}::{}", module_name, func_name);
                        if let Some(func) = self.functions.get(&qualified).cloned() {
                            // Also register with alias if present
                            if let Some(alias) = &import.alias {
                                let aliased = format!("{}::{}", alias, func_name);
                                self.functions.insert(aliased, func.clone());
                            }
                            // Register as unqualified for direct use after import
                            self.functions.insert(func_name, func);
                        }
                    }
                }
                Ok(())
            }
            TopLevel::Function(func) => {
                // Store function with qualified name if in a module context
                let qualified_name = match current_module {
                    Some(module) => format!("{}::{}", module, func.name),
                    None => func.name.clone(),
                };
                self.functions.insert(qualified_name, func.clone());

                // Also store with just the name for local access within module
                self.functions.insert(func.name.clone(), func.clone());
                Ok(())
            }
            TopLevel::Control(stmt) => {
                self.eval_control_stmt(stmt)?;
                Ok(())
            }
            // ExternCoproc blocks are handled before evaluation by the
            // coproc resolution pass (coproc::resolve_coproc_blocks).
            // Any surviving blocks have already been registered in
            // self.coproc_ns; nothing more to do here at execution time.
            TopLevel::ExternCoproc(_) => Ok(()),
        }
    }

    fn eval_control_stmt(&mut self, stmt: &ControlStmt) -> Result<Option<Value>> {
        self.check_iteration_limit()?;

        match stmt {
            ControlStmt::Assignment(assignment) => {
                let value = match &assignment.value {
                    Expr::Data(expr) => self.eval_data_expr(expr)?,
                    Expr::Control(expr) => self.eval_control_expr_to_value(expr)?,
                };

                self.add_trace("assignment", &format!("{} = {}", assignment.target, value));
                self.last_result = Some(value.clone());
                self.set_variable(assignment.target.clone(), value);
                Ok(None)
            }
            ControlStmt::If(if_stmt) => {
                let condition = self.eval_control_expr_to_value(&if_stmt.condition)?;

                self.add_trace("if", &format!("if {}", condition));

                if condition.is_truthy() {
                    for stmt in &if_stmt.then_branch {
                        if let Some(val) = self.eval_control_stmt(stmt)? {
                            return Ok(Some(val));
                        }
                    }
                } else if let Some(else_branch) = &if_stmt.else_branch {
                    for stmt in else_branch {
                        if let Some(val) = self.eval_control_stmt(stmt)? {
                            return Ok(Some(val));
                        }
                    }
                }
                Ok(None)
            }
            ControlStmt::While(while_stmt) => {
                self.add_trace("while", "entering while loop");

                while self
                    .eval_control_expr_to_value(&while_stmt.condition)?
                    .is_truthy()
                {
                    self.iteration_count += 1;
                    self.check_iteration_limit()?;

                    for stmt in &while_stmt.body {
                        if let Some(val) = self.eval_control_stmt(stmt)? {
                            return Ok(Some(val));
                        }
                    }
                }
                Ok(None)
            }
            ControlStmt::For(for_stmt) => {
                let range = &for_stmt.range;
                let start = self.eval_data_expr(&range.start)?;
                let end = self.eval_data_expr(&range.end)?;

                self.add_trace(
                    "for",
                    &format!("for {} in {}..{}", for_stmt.variable, start, end),
                );

                let (start_int, end_int) = match (start, end) {
                    (Value::Int(s), Value::Int(e)) => (s, e),
                    _ => return Err(JtvError::TypeError("Range must be integers".to_string())),
                };

                let step = if let Some(step_expr) = &range.step {
                    match self.eval_data_expr(step_expr)? {
                        Value::Int(s) => s,
                        _ => {
                            return Err(JtvError::TypeError("Step must be an integer".to_string()))
                        }
                    }
                } else {
                    1
                };

                let mut i = start_int;
                while (step > 0 && i < end_int) || (step < 0 && i > end_int) {
                    self.iteration_count += 1;
                    self.check_iteration_limit()?;

                    self.set_variable(for_stmt.variable.clone(), Value::Int(i));

                    for stmt in &for_stmt.body {
                        if let Some(val) = self.eval_control_stmt(stmt)? {
                            return Ok(Some(val));
                        }
                    }

                    i += step;
                }
                Ok(None)
            }
            ControlStmt::Return(expr) => {
                let value = if let Some(expr) = expr {
                    self.eval_data_expr(expr)?
                } else {
                    Value::Unit
                };
                self.add_trace("return", &format!("return {}", value));
                Ok(Some(value))
            }
            ControlStmt::Print(exprs) => {
                let mut output = String::new();
                for (i, expr) in exprs.iter().enumerate() {
                    if i > 0 {
                        output.push(' ');
                    }
                    let value = self.eval_data_expr(expr)?;
                    output.push_str(&format!("{}", value));
                }
                if self.capture_output {
                    self.output_buffer.push(output.clone());
                } else {
                    println!("{}", output);
                }
                self.add_trace("print", &output);
                Ok(None)
            }
            ControlStmt::ReverseBlock(block) => {
                self.eval_reverse_block(block)?;
                Ok(None)
            }
            ControlStmt::ReversibleBlock(stmt) => {
                self.eval_reversible_block(stmt)?;
                Ok(None)
            }
            ControlStmt::ReverseToken(tok_name) => {
                self.eval_reverse_token(tok_name)?;
                Ok(None)
            }
            ControlStmt::AbandonToken(tok_name) => {
                self.eval_abandon_token(tok_name)?;
                Ok(None)
            }
            ControlStmt::Block(stmts) => {
                for stmt in stmts {
                    if let Some(val) = self.eval_control_stmt(stmt)? {
                        return Ok(Some(val));
                    }
                }
                Ok(None)
            }
        }
    }

    fn eval_reverse_block(&mut self, block: &ReverseBlock) -> Result<()> {
        // JtV v2 semantics: `reverse { x += v }` IS subtraction.
        // Subtraction is not a grammar primitive; it arises from reversing addition.
        //
        // `execute_inverse` applies the INVERSE of each operation in reverse
        // declaration order.  For simple single-op blocks:
        //   reverse { x += 5 }  →  x = x - 5
        //   reverse { x -= 5 }  →  x = x + 5
        //
        // For multi-op blocks, inversion happens in reverse order so that
        // the combined effect is the mathematical inverse of the forward block.
        //
        // The CNO (Certified Null Operation) pattern uses a SEPARATE `reversible`
        // block (Phase 2 — not yet in the grammar) for the forward pass, then
        // `reverse` to undo.  The current `reverse { }` construct applies
        // inverses directly (no forward pass).
        use crate::reversible::ReversibleInterpreter;

        let mut rev_interp = ReversibleInterpreter::with_state(self.globals.clone());
        rev_interp.execute_inverse(block)?;

        for (name, value) in rev_interp.get_state() {
            self.set_variable(name.clone(), value.clone());
        }

        if self.trace_enabled {
            self.add_trace(
                "reverse_block",
                &format!("applied inverse of {} operations", block.body.len()),
            );
        }

        Ok(())
    }

    /// v2 — `reversible { stmts } -> tok`
    ///
    /// Runs the block forward using `execute_forward`, which records concrete
    /// `RecordedOp` values.  The log is stored in `token_store` and the token
    /// variable (if any) is bound to `Value::ReversalToken(id)`.
    fn eval_reversible_block(&mut self, stmt: &ReversibleBlockStmt) -> Result<()> {
        use crate::reversible::ReversibleInterpreter;

        let mut rev = ReversibleInterpreter::with_state(self.globals.clone());
        rev.execute_forward(&crate::ast::ReverseBlock {
            body: stmt.body.clone(),
        })?;

        // Sync the forward-pass state back to globals.
        for (name, value) in rev.get_state() {
            self.set_variable(name.clone(), value.clone());
        }

        // Store the log and optionally bind the token.
        let id = self.next_token_id;
        self.next_token_id += 1;
        self.token_store.insert(id, rev.take_recorded_ops());

        if let Some(tok_name) = &stmt.token_binding {
            self.set_variable(tok_name.clone(), Value::ReversalToken(id));
        }
        // If no binding, the log is in the store but inaccessible — effectively abandoned.

        if self.trace_enabled {
            self.add_trace("reversible_block", &format!("forward pass; token #{}", id));
        }
        Ok(())
    }

    /// v2 — `reverse tok`
    ///
    /// Looks up the token, retrieves the operation log, applies the inverses
    /// in reverse order to the current state, then removes the token (linear
    /// consumption guarantees no double-reversal).
    fn eval_reverse_token(&mut self, tok_name: &str) -> Result<()> {
        use crate::reversible::ReversibleInterpreter;

        let tok_val = self.get_variable(tok_name)?;
        let id = match tok_val {
            Value::ReversalToken(id) => id,
            other => {
                return Err(JtvError::TypeError(format!(
                    "`reverse` requires a ReversalToken, got {}",
                    other
                )))
            }
        };

        let ops = self.token_store.remove(&id).ok_or_else(|| {
            JtvError::RuntimeError(format!(
                "reversal token #{} already consumed or not found",
                id
            ))
        })?;

        // Remove the token variable (it's been consumed).
        self.globals.remove(tok_name);

        // Apply inverses: build a ReverseTrace from the recorded ops and replay.
        let mut rev = ReversibleInterpreter::with_state(self.globals.clone());
        rev.apply_inverse_ops(&ops)?;

        for (name, value) in rev.get_state() {
            self.set_variable(name.clone(), value.clone());
        }

        if self.trace_enabled {
            self.add_trace("reverse_token", &format!("consumed token #{}", id));
        }
        Ok(())
    }

    /// v2 — `abandon tok`
    ///
    /// Discards the operation log without applying inverses.  The forward state
    /// is already in globals (committed during `reversible { }`).
    /// Removes the token variable (linear consumption).
    fn eval_abandon_token(&mut self, tok_name: &str) -> Result<()> {
        let tok_val = self.get_variable(tok_name)?;
        let id = match tok_val {
            Value::ReversalToken(id) => id,
            other => {
                return Err(JtvError::TypeError(format!(
                    "`abandon` requires a ReversalToken, got {}",
                    other
                )))
            }
        };

        self.token_store.remove(&id);
        self.globals.remove(tok_name);

        if self.trace_enabled {
            self.add_trace("abandon_token", &format!("discarded token #{}", id));
        }
        Ok(())
    }

    fn eval_data_expr(&mut self, expr: &DataExpr) -> Result<Value> {
        match expr {
            DataExpr::Number(num) => Value::from_number(num),
            DataExpr::StringLit(s) => Ok(Value::String(s.clone())),
            DataExpr::Identifier(name) => self.get_variable(name),
            DataExpr::Add(left, right) => {
                let left_val = self.eval_data_expr(left)?;
                let right_val = self.eval_data_expr(right)?;
                left_val.add(&right_val)
            }
            DataExpr::Negate(expr) => {
                let value = self.eval_data_expr(expr)?;
                value.negate()
            }
            DataExpr::FunctionCall(call) => self.eval_function_call(call),
            DataExpr::List(elements) => {
                let mut values = Vec::new();
                for elem in elements {
                    values.push(self.eval_data_expr(elem)?);
                }
                Ok(Value::List(values))
            }
            DataExpr::Tuple(elements) => {
                let mut values = Vec::new();
                for elem in elements {
                    values.push(self.eval_data_expr(elem)?);
                }
                Ok(Value::Tuple(values))
            }
        }
    }

    fn eval_control_expr_to_value(&mut self, expr: &ControlExpr) -> Result<Value> {
        match expr {
            ControlExpr::Data(data_expr) => self.eval_data_expr(data_expr),
            ControlExpr::Comparison(left, op, right) => {
                let left_val = self.eval_data_expr(left)?;
                let right_val = self.eval_data_expr(right)?;

                let result = match op {
                    Comparator::Eq => left_val.eq(&right_val)?,
                    Comparator::Ne => left_val.ne(&right_val)?,
                    Comparator::Lt => left_val.lt(&right_val)?,
                    Comparator::Le => left_val.le(&right_val)?,
                    Comparator::Gt => left_val.gt(&right_val)?,
                    Comparator::Ge => left_val.ge(&right_val)?,
                };

                Ok(Value::Bool(result))
            }
            ControlExpr::Logical(left, op, right) => {
                let left_val = self.eval_control_expr_to_value(left)?;

                match op {
                    LogicalOp::And => {
                        if !left_val.is_truthy() {
                            Ok(Value::Bool(false))
                        } else {
                            let right_val = self.eval_control_expr_to_value(right)?;
                            Ok(Value::Bool(right_val.is_truthy()))
                        }
                    }
                    LogicalOp::Or => {
                        if left_val.is_truthy() {
                            Ok(Value::Bool(true))
                        } else {
                            let right_val = self.eval_control_expr_to_value(right)?;
                            Ok(Value::Bool(right_val.is_truthy()))
                        }
                    }
                }
            }
            ControlExpr::Not(expr) => {
                let value = self.eval_control_expr_to_value(expr)?;
                Ok(Value::Bool(!value.is_truthy()))
            }
        }
    }

    fn eval_function_call(&mut self, call: &FunctionCall) -> Result<Value> {
        // Try qualified name first (e.g., "Math::add")
        let qualified = call.qualified_name();

        // Look up function: try qualified name, then unqualified
        let func = self
            .functions
            .get(&qualified)
            .or_else(|| self.functions.get(&call.name));

        // Native impl registered (lowering complete): dispatch directly.
        if func.is_none() {
            if let Some(native) = self.native_impls.get(&call.name).cloned() {
                let mut arg_values = Vec::with_capacity(call.args.len());
                for arg in &call.args {
                    arg_values.push(self.eval_data_expr(arg)?);
                }
                return native(&arg_values);
            }
        }

        // Phase-boundary error: function is a live extern coproc entry but
        // native lowering is not yet implemented (per JtV ADR-0005).
        if func.is_none() {
            if let Some(entry) = self.coproc_ns.get(&call.name) {
                return Err(JtvError::ExternCoprocNotYetLowered {
                    gate: entry.gate_name.clone(),
                    name: call.name.clone(),
                });
            }
        }

        let func = func
            .ok_or_else(|| JtvError::UndefinedFunction(qualified.clone()))?
            .clone();

        if func.params.len() != call.args.len() {
            return Err(JtvError::ArityMismatch {
                expected: func.params.len(),
                got: call.args.len(),
            });
        }

        // Evaluate arguments
        let mut arg_values = Vec::new();
        for arg in &call.args {
            arg_values.push(self.eval_data_expr(arg)?);
        }

        // Create new scope
        self.call_stack.push(HashMap::new());

        // Bind parameters
        for (param, value) in func.params.iter().zip(arg_values.iter()) {
            self.set_variable(param.name.clone(), value.clone());
        }

        // Execute function body
        let mut result = Value::Unit;
        for stmt in &func.body {
            if let Some(val) = self.eval_control_stmt(stmt)? {
                result = val;
                break;
            }
        }

        // Pop scope
        self.call_stack.pop();

        Ok(result)
    }

    pub fn get_variable(&self, name: &str) -> Result<Value> {
        // Check call stack (local variables)
        for scope in self.call_stack.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Ok(value.clone());
            }
        }

        // Check globals
        self.globals
            .get(name)
            .cloned()
            .ok_or_else(|| JtvError::UndefinedVariable(name.to_string()))
    }

    fn set_variable(&mut self, name: String, value: Value) {
        if let Some(scope) = self.call_stack.last_mut() {
            scope.insert(name, value);
        } else {
            self.globals.insert(name, value);
        }
    }

    fn check_iteration_limit(&self) -> Result<()> {
        if self.iteration_count > MAX_ITERATIONS {
            Err(JtvError::MaxIterationsExceeded)
        } else {
            Ok(())
        }
    }

    pub fn reset_iteration_count(&mut self) {
        self.iteration_count = 0;
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_program;

    #[test]
    fn test_simple_addition() {
        let code = "x = 5 + 3";
        let program = parse_program(code).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.run(&program).unwrap();

        let x = interpreter.get_variable("x").unwrap();
        assert_eq!(x, Value::Int(8));
    }

    #[test]
    fn test_function_call() {
        // Test function definition and call
        let code = r#"
fn add(a: Int, b: Int): Int {
    return a + b
}
result = add(5, 3)
        "#;

        let program = parse_program(code).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.run(&program).unwrap();

        let result = interpreter.get_variable("result").unwrap();
        assert_eq!(result, Value::Int(8));
    }

    #[test]
    fn test_function_return() {
        // Test function with return statement (no call in assignment)
        let code = r#"
fn double(x: Int): Int {
    return x + x
}
        "#;

        let program = parse_program(code).unwrap();
        let mut interpreter = Interpreter::new();
        // Just test that it parses and runs without calling
        interpreter.run(&program).unwrap();
    }

    #[test]
    fn test_for_loop() {
        let code = r#"
            sum = 0
            for i in 1..6 {
                sum = sum + i
            }
        "#;

        let program = parse_program(code).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.run(&program).unwrap();

        let sum = interpreter.get_variable("sum").unwrap();
        assert_eq!(sum, Value::Int(15)); // 1+2+3+4+5
    }
}

// ===========================================================================
// Denotational correspondence (PROOF-2 / gap-001)
//
// Bridges this interpreter's `eval_data_expr` to the Lean denotational model
// `evalDataExpr` in jtv_proofs/JtvCore.lean, over the integer fragment the
// model covers:  DataExpr = lit | var | add | neg  evaluated against a state
// σ : String → Int.
//
// `denot` re-encodes the four Lean rules exactly (in i128, giving Lean's
// unbounded ℤ headroom). We then EXHAUSTIVELY enumerate every integer-fragment
// expression up to height 2 over a small literal/variable set, across several
// states, and assert the interpreter agrees with `denot` on all of them.
//
// Honest bound:
//   (1) integer fragment only — String/Float/Rational/Complex/Hex/Binary/
//       Symbolic literals and FunctionCall/List/Tuple are outside the model;
//   (2) within i64 range — `Value::Int` is i64 (`checked_add`/`checked_neg`),
//       Lean's ℤ is unbounded, so they agree exactly where no i64 overflow
//       occurs; the enumeration stays in range by construction, and
//       `overflow_is_the_correspondence_boundary` pins the edge;
//   (3) this is exhaustive-up-to-depth correspondence, NOT a Lean-mechanised
//       refinement of the Rust evaluator (that is the deeper PROOF-2 rung).
// ===========================================================================
#[cfg(test)]
mod denotational_correspondence {
    use super::*;
    use crate::ast::{DataExpr, Number};
    use std::collections::HashMap;

    /// The Lean `evalDataExpr` rules, re-encoded in i128.
    /// `None` for any node outside the modelled integer fragment.
    fn denot(e: &DataExpr, sigma: &HashMap<String, i128>) -> Option<i128> {
        match e {
            DataExpr::Number(Number::Int(n)) => Some(*n as i128),
            DataExpr::Identifier(x) => Some(*sigma.get(x).unwrap_or(&0)),
            DataExpr::Add(l, r) => Some(denot(l, sigma)? + denot(r, sigma)?),
            DataExpr::Negate(inner) => Some(-denot(inner, sigma)?),
            _ => None,
        }
    }

    /// Every integer-fragment expression of height ≤ `height`.
    fn enumerate(height: usize, lits: &[i64], vars: &[&str]) -> Vec<DataExpr> {
        let mut out: Vec<DataExpr> = Vec::new();
        for &n in lits {
            out.push(DataExpr::Number(Number::Int(n)));
        }
        for &v in vars {
            out.push(DataExpr::Identifier(v.to_string()));
        }
        if height > 0 {
            let sub = enumerate(height - 1, lits, vars);
            for e in &sub {
                out.push(DataExpr::Negate(Box::new(e.clone())));
            }
            for a in &sub {
                for b in &sub {
                    out.push(DataExpr::Add(Box::new(a.clone()), Box::new(b.clone())));
                }
            }
        }
        out
    }

    /// A fresh interpreter with the given state σ seeded as integer variables.
    fn seed(state: &HashMap<String, i64>) -> Interpreter {
        let mut interp = Interpreter::new();
        for (k, v) in state {
            interp.set_variable(k.clone(), Value::Int(*v));
        }
        interp
    }

    #[test]
    fn interpreter_matches_denotational_model_on_int_fragment() {
        let lits = [-2i64, -1, 0, 1, 2];
        let vars = ["x", "y"];
        let states: Vec<HashMap<String, i64>> = vec![
            HashMap::from([("x".to_string(), 0i64), ("y".to_string(), 0i64)]),
            HashMap::from([("x".to_string(), 3i64), ("y".to_string(), -1i64)]),
            HashMap::from([("x".to_string(), -2i64), ("y".to_string(), 2i64)]),
        ];
        let exprs = enumerate(2, &lits, &vars);

        let mut checked = 0u64;
        for state in &states {
            let mut interp = seed(state);
            let sigma: HashMap<String, i128> =
                state.iter().map(|(k, v)| (k.clone(), *v as i128)).collect();
            for e in &exprs {
                let expected = denot(e, &sigma)
                    .expect("every enumerated expression is in the integer fragment");
                // The corpus is constructed to stay within i64 range.
                assert!(
                    expected >= i64::MIN as i128 && expected <= i64::MAX as i128,
                    "corpus left i64 range ({expected}); shrink literals or height"
                );
                match interp.eval_data_expr(e) {
                    Ok(Value::Int(n)) => assert_eq!(
                        n as i128, expected,
                        "interpreter vs denotation disagree on {e:?} under {state:?}"
                    ),
                    other => panic!(
                        "expected Ok(Int({expected})) for {e:?} under {state:?}, got {other:?}"
                    ),
                }
                checked += 1;
            }
        }
        // height-2 over 7 leaves = 4039 expressions × 3 states.
        assert!(
            checked >= 12_000,
            "expected a substantial corpus, checked only {checked}"
        );
    }

    #[test]
    fn overflow_is_the_correspondence_boundary() {
        // i64::MAX + 1 is 2^63 in ℤ — out of i64 range. The interpreter must
        // SIGNAL overflow (checked_add), never wrap, so the correspondence
        // boundary is exactly i64 range.
        let e = DataExpr::Add(
            Box::new(DataExpr::Number(Number::Int(i64::MAX))),
            Box::new(DataExpr::Number(Number::Int(1))),
        );
        let mut interp = Interpreter::new();
        let sigma = HashMap::new();
        let denotational = denot(&e, &sigma).unwrap();
        assert!(
            denotational > i64::MAX as i128,
            "this case is meant to exceed i64"
        );
        assert!(
            interp.eval_data_expr(&e).is_err(),
            "interpreter must error where ℤ leaves i64 range, not wrap"
        );

        // neg(i64::MIN) is 2^63 in ℤ — also out of range → must error too.
        let e2 = DataExpr::Negate(Box::new(DataExpr::Number(Number::Int(i64::MIN))));
        assert!(
            interp.eval_data_expr(&e2).is_err(),
            "neg(i64::MIN) must error (checked_neg), not wrap"
        );
    }

    #[test]
    fn shared_golden_corpus_matches_lean() {
        // EXACTLY mirrors the golden `example`s in jtv_proofs/JtvCore.lean —
        // each (expr, σ, value) is pinned by `rfl` on the Lean side.
        let lit = |n: i64| DataExpr::Number(Number::Int(n));
        let var = |s: &str| DataExpr::Identifier(s.to_string());
        let add = |a: DataExpr, b: DataExpr| DataExpr::Add(Box::new(a), Box::new(b));
        let neg = |a: DataExpr| DataExpr::Negate(Box::new(a));

        let cases: Vec<(DataExpr, HashMap<String, i64>, i64)> = vec![
            (add(lit(2), lit(3)), HashMap::new(), 5),
            (neg(lit(5)), HashMap::new(), -5),
            (neg(add(lit(1), lit(2))), HashMap::new(), -3),
            (add(neg(lit(2)), lit(5)), HashMap::new(), 3),
            (
                add(var("x"), lit(1)),
                HashMap::from([("x".to_string(), 4i64)]),
                5,
            ),
            (
                add(var("x"), neg(var("x"))),
                HashMap::from([("x".to_string(), 7i64)]),
                0,
            ),
            (
                add(add(var("x"), var("y")), lit(1)),
                HashMap::from([("x".to_string(), 3i64), ("y".to_string(), 4i64)]),
                8,
            ),
        ];

        for (e, state, expected) in &cases {
            let sigma: HashMap<String, i128> =
                state.iter().map(|(k, v)| (k.clone(), *v as i128)).collect();
            assert_eq!(
                denot(e, &sigma),
                Some(*expected as i128),
                "denotational reference mismatch on {e:?}"
            );
            let mut interp = seed(state);
            assert_eq!(
                interp.eval_data_expr(e).unwrap(),
                Value::Int(*expected),
                "interpreter mismatch on {e:?}"
            );
        }
    }
}
