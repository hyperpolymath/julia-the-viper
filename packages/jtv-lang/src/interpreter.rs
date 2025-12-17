// Interpreter for Julia the Viper
use crate::ast::*;
use crate::number::Value;
use crate::error::{JtvError, Result};
use std::collections::HashMap;

const MAX_ITERATIONS: usize = 1_000_000; // Safety limit for loops

pub struct Interpreter {
    globals: HashMap<String, Value>,
    functions: HashMap<String, FunctionDecl>,
    call_stack: Vec<HashMap<String, Value>>,
    iteration_count: usize,
    trace_enabled: bool,
    trace: Vec<TraceEntry>,
}

#[derive(Debug, Clone)]
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
            call_stack: vec![],
            iteration_count: 0,
            trace_enabled: false,
            trace: vec![],
        }
    }

    pub fn enable_trace(&mut self) {
        self.trace_enabled = true;
    }

    pub fn get_trace(&self) -> &[TraceEntry] {
        &self.trace
    }

    fn add_trace(&mut self, stmt_type: &str, line: &str) {
        if self.trace_enabled {
            let env: HashMap<String, String> = self.globals
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
        for statement in &program.statements {
            self.eval_top_level(statement)?;
        }
        Ok(())
    }

    fn eval_top_level(&mut self, top_level: &TopLevel) -> Result<()> {
        match top_level {
            TopLevel::Module(module) => {
                for stmt in &module.body {
                    self.eval_top_level(stmt)?;
                }
                Ok(())
            }
            TopLevel::Import(_) => {
                // Import handling would go here
                Ok(())
            }
            TopLevel::Function(func) => {
                self.functions.insert(func.name.clone(), func.clone());
                Ok(())
            }
            TopLevel::Control(stmt) => {
                self.eval_control_stmt(stmt)?;
                Ok(())
            }
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

                while self.eval_control_expr_to_value(&while_stmt.condition)?.is_truthy() {
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

                self.add_trace("for", &format!("for {} in {}..{}", for_stmt.variable, start, end));

                let (start_int, end_int) = match (start, end) {
                    (Value::Int(s), Value::Int(e)) => (s, e),
                    _ => return Err(JtvError::TypeError("Range must be integers".to_string())),
                };

                let step = if let Some(step_expr) = &range.step {
                    match self.eval_data_expr(step_expr)? {
                        Value::Int(s) => s,
                        _ => return Err(JtvError::TypeError("Step must be an integer".to_string())),
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
                println!("{}", output);
                self.add_trace("print", &output);
                Ok(None)
            }
            ControlStmt::ReverseBlock(block) => {
                self.eval_reverse_block(block)?;
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
        // Forward execution
        for stmt in &block.body {
            match stmt {
                ReversibleStmt::AddAssign(target, expr) => {
                    let value = self.eval_data_expr(expr)?;
                    let current = self.get_variable(target)?;
                    let new_value = current.add(&value)?;
                    self.set_variable(target.clone(), new_value);
                }
                ReversibleStmt::SubAssign(target, expr) => {
                    let value = self.eval_data_expr(expr)?;
                    let current = self.get_variable(target)?;
                    let neg_value = value.negate()?;
                    let new_value = current.add(&neg_value)?;
                    self.set_variable(target.clone(), new_value);
                }
                ReversibleStmt::If(if_stmt) => {
                    let condition = self.eval_control_expr_to_value(&if_stmt.condition)?;
                    if condition.is_truthy() {
                        for stmt in &if_stmt.then_branch {
                            self.eval_control_stmt(stmt)?;
                        }
                    } else if let Some(else_branch) = &if_stmt.else_branch {
                        for stmt in else_branch {
                            self.eval_control_stmt(stmt)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn eval_data_expr(&mut self, expr: &DataExpr) -> Result<Value> {
        match expr {
            DataExpr::Number(num) => Value::from_number(num),
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
        let func = self.functions.get(&call.name)
            .ok_or_else(|| JtvError::UndefinedFunction(call.name.clone()))?
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

    fn get_variable(&self, name: &str) -> Result<Value> {
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
        // Test function definition and direct execution
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
