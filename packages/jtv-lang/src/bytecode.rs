// SPDX-License-Identifier: MIT OR GPL-3.0-or-later OR Palimpsest-0.8
// SPDX-FileCopyrightText: 2025 Julia the Viper Contributors
//
// Julia the Viper - Bytecode IR for compilation backends

use crate::ast::*;
use crate::error::{JtvError, Result};
use std::collections::HashMap;

/// Bytecode instructions for the JtV virtual machine
#[derive(Debug, Clone, PartialEq)]
pub enum Opcode {
    // Stack operations
    Push(Value),           // Push value onto stack
    Pop,                   // Pop top of stack
    Dup,                   // Duplicate top of stack

    // Variable operations
    LoadLocal(u32),        // Load local variable by index
    StoreLocal(u32),       // Store to local variable
    LoadGlobal(u32),       // Load global variable by index
    StoreGlobal(u32),      // Store to global variable

    // Arithmetic (addition-only in Data Language)
    Add,                   // Pop two values, push sum
    Neg,                   // Negate top of stack

    // Comparison operations
    Eq,                    // Equal
    Ne,                    // Not equal
    Lt,                    // Less than
    Le,                    // Less or equal
    Gt,                    // Greater than
    Ge,                    // Greater or equal

    // Logical operations
    And,                   // Logical AND
    Or,                    // Logical OR
    Not,                   // Logical NOT

    // Control flow
    Jump(u32),             // Unconditional jump to instruction
    JumpIfFalse(u32),      // Jump if top of stack is false
    JumpIfTrue(u32),       // Jump if top of stack is true

    // Function operations
    Call(u32),             // Call function by index
    Return,                // Return from function

    // Built-in operations
    Print,                 // Print top of stack

    // Collection operations
    MakeList(u32),         // Create list from n stack values
    MakeTuple(u32),        // Create tuple from n stack values

    // Reversible operations (for reverse blocks)
    BeginReverse,          // Mark start of reversible section
    EndReverse,            // Mark end of reversible section

    // Halt
    Halt,                  // Stop execution
}

/// Runtime value representation
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Rational(i64, i64),
    Complex(f64, f64),
    Bool(bool),
    String(String),
    List(Vec<Value>),
    Tuple(Vec<Value>),
    Unit,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{}", n),
            Value::Float(n) => write!(f, "{}", n),
            Value::Rational(num, denom) => write!(f, "{}/{}", num, denom),
            Value::Complex(re, im) => {
                if *im >= 0.0 {
                    write!(f, "{}+{}i", re, im)
                } else {
                    write!(f, "{}{}i", re, im)
                }
            }
            Value::Bool(b) => write!(f, "{}", b),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::List(items) => {
                write!(f, "[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            Value::Tuple(items) => {
                write!(f, "(")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", item)?;
                }
                write!(f, ")")
            }
            Value::Unit => write!(f, "()"),
        }
    }
}

/// A compiled function
#[derive(Debug, Clone)]
pub struct CompiledFunction {
    pub name: String,
    pub arity: usize,
    pub locals: usize,
    pub code: Vec<Opcode>,
}

/// A compiled module/program
#[derive(Debug, Clone)]
pub struct CompiledModule {
    pub functions: Vec<CompiledFunction>,
    pub globals: Vec<String>,
    pub entry_point: usize,  // Index of main/entry function
    pub code: Vec<Opcode>,   // Top-level code
}

/// Bytecode compiler
pub struct BytecodeCompiler {
    module: CompiledModule,
    local_vars: HashMap<String, u32>,
    global_vars: HashMap<String, u32>,
    function_indices: HashMap<String, u32>,
    next_local: u32,
}

impl BytecodeCompiler {
    pub fn new() -> Self {
        BytecodeCompiler {
            module: CompiledModule {
                functions: vec![],
                globals: vec![],
                entry_point: 0,
                code: vec![],
            },
            local_vars: HashMap::new(),
            global_vars: HashMap::new(),
            function_indices: HashMap::new(),
            next_local: 0,
        }
    }

    /// Compile a program to bytecode
    pub fn compile(&mut self, program: &Program) -> Result<CompiledModule> {
        // First pass: register all functions
        for stmt in &program.statements {
            if let TopLevel::Function(func) = stmt {
                let idx = self.module.functions.len() as u32;
                self.function_indices.insert(func.name.clone(), idx);
                // Add placeholder
                self.module.functions.push(CompiledFunction {
                    name: func.name.clone(),
                    arity: func.params.len(),
                    locals: 0,
                    code: vec![],
                });
            }
        }

        // Second pass: compile functions and top-level code
        for stmt in &program.statements {
            match stmt {
                TopLevel::Function(func) => {
                    self.compile_function(func)?;
                }
                TopLevel::Control(ctrl) => {
                    self.compile_control_stmt(ctrl, &mut self.module.code.clone())?;
                }
                TopLevel::Module(module) => {
                    for inner in &module.body {
                        if let TopLevel::Control(ctrl) = inner {
                            self.compile_control_stmt(ctrl, &mut self.module.code.clone())?;
                        }
                    }
                }
                TopLevel::Import(_) => {
                    // Imports handled at link time
                }
            }
        }

        self.module.code.push(Opcode::Halt);
        Ok(self.module.clone())
    }

    fn compile_function(&mut self, func: &FunctionDecl) -> Result<()> {
        let mut code = vec![];
        self.local_vars.clear();
        self.next_local = 0;

        // Register parameters as locals
        for param in &func.params {
            let idx = self.next_local;
            self.local_vars.insert(param.name.clone(), idx);
            self.next_local += 1;
        }

        // Compile body
        for stmt in &func.body {
            self.compile_control_stmt(stmt, &mut code)?;
        }

        // Ensure function returns
        code.push(Opcode::Return);

        // Update function in module
        if let Some(&idx) = self.function_indices.get(&func.name) {
            self.module.functions[idx as usize].code = code;
            self.module.functions[idx as usize].locals = self.next_local as usize;
        }

        Ok(())
    }

    fn compile_control_stmt(&mut self, stmt: &ControlStmt, code: &mut Vec<Opcode>) -> Result<()> {
        match stmt {
            ControlStmt::Assignment(assign) => {
                self.compile_expr(&assign.value, code)?;
                let idx = self.get_or_create_var(&assign.target);
                code.push(Opcode::StoreLocal(idx));
            }
            ControlStmt::If(if_stmt) => {
                self.compile_control_expr(&if_stmt.condition, code)?;
                let jump_else = code.len();
                code.push(Opcode::JumpIfFalse(0)); // Placeholder

                // Then branch
                for s in &if_stmt.then_branch {
                    self.compile_control_stmt(s, code)?;
                }

                if let Some(else_branch) = &if_stmt.else_branch {
                    let jump_end = code.len();
                    code.push(Opcode::Jump(0)); // Placeholder

                    // Patch else jump
                    code[jump_else] = Opcode::JumpIfFalse(code.len() as u32);

                    // Else branch
                    for s in else_branch {
                        self.compile_control_stmt(s, code)?;
                    }

                    // Patch end jump
                    code[jump_end] = Opcode::Jump(code.len() as u32);
                } else {
                    // Patch else jump to end
                    code[jump_else] = Opcode::JumpIfFalse(code.len() as u32);
                }
            }
            ControlStmt::While(while_stmt) => {
                let loop_start = code.len();
                self.compile_control_expr(&while_stmt.condition, code)?;
                let jump_end = code.len();
                code.push(Opcode::JumpIfFalse(0)); // Placeholder

                for s in &while_stmt.body {
                    self.compile_control_stmt(s, code)?;
                }

                code.push(Opcode::Jump(loop_start as u32));
                code[jump_end] = Opcode::JumpIfFalse(code.len() as u32);
            }
            ControlStmt::For(for_stmt) => {
                // Compile range start
                self.compile_data_expr(&for_stmt.range.start, code)?;
                let iter_var = self.get_or_create_var(&for_stmt.variable);
                code.push(Opcode::StoreLocal(iter_var));

                let loop_start = code.len();

                // Check condition: iter < end
                code.push(Opcode::LoadLocal(iter_var));
                self.compile_data_expr(&for_stmt.range.end, code)?;
                code.push(Opcode::Lt);

                let jump_end = code.len();
                code.push(Opcode::JumpIfFalse(0)); // Placeholder

                // Body
                for s in &for_stmt.body {
                    self.compile_control_stmt(s, code)?;
                }

                // Increment: iter = iter + step (default 1)
                code.push(Opcode::LoadLocal(iter_var));
                if let Some(step) = &for_stmt.range.step {
                    self.compile_data_expr(step, code)?;
                } else {
                    code.push(Opcode::Push(Value::Int(1)));
                }
                code.push(Opcode::Add);
                code.push(Opcode::StoreLocal(iter_var));

                code.push(Opcode::Jump(loop_start as u32));
                code[jump_end] = Opcode::JumpIfFalse(code.len() as u32);
            }
            ControlStmt::Return(expr) => {
                if let Some(e) = expr {
                    self.compile_data_expr(e, code)?;
                } else {
                    code.push(Opcode::Push(Value::Unit));
                }
                code.push(Opcode::Return);
            }
            ControlStmt::Print(exprs) => {
                for e in exprs {
                    self.compile_data_expr(e, code)?;
                    code.push(Opcode::Print);
                }
            }
            ControlStmt::ReverseBlock(block) => {
                code.push(Opcode::BeginReverse);
                for s in &block.body {
                    self.compile_reversible_stmt(s, code)?;
                }
                code.push(Opcode::EndReverse);
            }
            ControlStmt::Block(stmts) => {
                for s in stmts {
                    self.compile_control_stmt(s, code)?;
                }
            }
        }
        Ok(())
    }

    fn compile_reversible_stmt(&mut self, stmt: &ReversibleStmt, code: &mut Vec<Opcode>) -> Result<()> {
        match stmt {
            ReversibleStmt::AddAssign(target, expr) => {
                let idx = self.get_or_create_var(target);
                code.push(Opcode::LoadLocal(idx));
                self.compile_data_expr(expr, code)?;
                code.push(Opcode::Add);
                code.push(Opcode::StoreLocal(idx));
            }
            ReversibleStmt::SubAssign(target, expr) => {
                let idx = self.get_or_create_var(target);
                code.push(Opcode::LoadLocal(idx));
                self.compile_data_expr(expr, code)?;
                code.push(Opcode::Neg);
                code.push(Opcode::Add);
                code.push(Opcode::StoreLocal(idx));
            }
            ReversibleStmt::If(if_stmt) => {
                self.compile_control_expr(&if_stmt.condition, code)?;
                let jump_else = code.len();
                code.push(Opcode::JumpIfFalse(0));

                for s in &if_stmt.then_branch {
                    self.compile_control_stmt(s, code)?;
                }

                if let Some(else_branch) = &if_stmt.else_branch {
                    let jump_end = code.len();
                    code.push(Opcode::Jump(0));
                    code[jump_else] = Opcode::JumpIfFalse(code.len() as u32);

                    for s in else_branch {
                        self.compile_control_stmt(s, code)?;
                    }
                    code[jump_end] = Opcode::Jump(code.len() as u32);
                } else {
                    code[jump_else] = Opcode::JumpIfFalse(code.len() as u32);
                }
            }
        }
        Ok(())
    }

    fn compile_expr(&mut self, expr: &Expr, code: &mut Vec<Opcode>) -> Result<()> {
        match expr {
            Expr::Data(data) => self.compile_data_expr(data, code),
            Expr::Control(ctrl) => self.compile_control_expr(ctrl, code),
        }
    }

    fn compile_data_expr(&mut self, expr: &DataExpr, code: &mut Vec<Opcode>) -> Result<()> {
        match expr {
            DataExpr::Number(num) => {
                let value = self.number_to_value(num);
                code.push(Opcode::Push(value));
            }
            DataExpr::Identifier(name) => {
                if let Some(&idx) = self.local_vars.get(name) {
                    code.push(Opcode::LoadLocal(idx));
                } else if let Some(&idx) = self.global_vars.get(name) {
                    code.push(Opcode::LoadGlobal(idx));
                } else {
                    return Err(JtvError::UndefinedVariable(name.clone()));
                }
            }
            DataExpr::Add(left, right) => {
                self.compile_data_expr(left, code)?;
                self.compile_data_expr(right, code)?;
                code.push(Opcode::Add);
            }
            DataExpr::Negate(inner) => {
                self.compile_data_expr(inner, code)?;
                code.push(Opcode::Neg);
            }
            DataExpr::FunctionCall(call) => {
                // Push arguments
                for arg in &call.args {
                    self.compile_data_expr(arg, code)?;
                }

                // Call function
                if let Some(&idx) = self.function_indices.get(&call.name) {
                    code.push(Opcode::Call(idx));
                } else {
                    return Err(JtvError::UndefinedFunction(call.name.clone()));
                }
            }
            DataExpr::List(elements) => {
                for e in elements {
                    self.compile_data_expr(e, code)?;
                }
                code.push(Opcode::MakeList(elements.len() as u32));
            }
            DataExpr::Tuple(elements) => {
                for e in elements {
                    self.compile_data_expr(e, code)?;
                }
                code.push(Opcode::MakeTuple(elements.len() as u32));
            }
        }
        Ok(())
    }

    fn compile_control_expr(&mut self, expr: &ControlExpr, code: &mut Vec<Opcode>) -> Result<()> {
        match expr {
            ControlExpr::Data(data) => self.compile_data_expr(data, code),
            ControlExpr::Comparison(left, op, right) => {
                self.compile_data_expr(left, code)?;
                self.compile_data_expr(right, code)?;
                code.push(match op {
                    Comparator::Eq => Opcode::Eq,
                    Comparator::Ne => Opcode::Ne,
                    Comparator::Lt => Opcode::Lt,
                    Comparator::Le => Opcode::Le,
                    Comparator::Gt => Opcode::Gt,
                    Comparator::Ge => Opcode::Ge,
                });
                Ok(())
            }
            ControlExpr::Logical(left, op, right) => {
                self.compile_control_expr(left, code)?;
                self.compile_control_expr(right, code)?;
                code.push(match op {
                    LogicalOp::And => Opcode::And,
                    LogicalOp::Or => Opcode::Or,
                });
                Ok(())
            }
            ControlExpr::Not(inner) => {
                self.compile_control_expr(inner, code)?;
                code.push(Opcode::Not);
                Ok(())
            }
        }
    }

    fn get_or_create_var(&mut self, name: &str) -> u32 {
        if let Some(&idx) = self.local_vars.get(name) {
            idx
        } else {
            let idx = self.next_local;
            self.local_vars.insert(name.to_string(), idx);
            self.next_local += 1;
            idx
        }
    }

    fn number_to_value(&self, num: &Number) -> Value {
        match num {
            Number::Int(n) => Value::Int(*n),
            Number::Float(f) => Value::Float(*f),
            Number::Rational(n, d) => Value::Rational(*n, *d),
            Number::Complex(r, i) => Value::Complex(*r, *i),
            Number::Hex(s) => {
                let n = i64::from_str_radix(s.trim_start_matches("0x"), 16).unwrap_or(0);
                Value::Int(n)
            }
            Number::Binary(s) => {
                let n = i64::from_str_radix(s.trim_start_matches("0b"), 2).unwrap_or(0);
                Value::Int(n)
            }
            Number::Symbolic(s) => Value::String(s.clone()),
        }
    }
}

impl Default for BytecodeCompiler {
    fn default() -> Self {
        Self::new()
    }
}

/// Bytecode VM for execution
pub struct BytecodeVM {
    stack: Vec<Value>,
    locals: Vec<Value>,
    globals: Vec<Value>,
    call_stack: Vec<(usize, usize, usize)>,  // (return_addr, locals_base, prev_locals_count)
    ip: usize,
}

impl BytecodeVM {
    pub fn new() -> Self {
        BytecodeVM {
            stack: vec![],
            locals: vec![],
            globals: vec![],
            call_stack: vec![],
            ip: 0,
        }
    }

    pub fn execute(&mut self, module: &CompiledModule) -> Result<Option<Value>> {
        self.ip = 0;
        self.stack.clear();
        self.locals.clear();
        self.globals.clear();

        // Initialize globals
        for _ in &module.globals {
            self.globals.push(Value::Unit);
        }

        // Execute top-level code
        while self.ip < module.code.len() {
            if !self.execute_instruction(&module.code[self.ip], module)? {
                break;
            }
            self.ip += 1;
        }

        Ok(self.stack.pop())
    }

    fn execute_instruction(&mut self, op: &Opcode, module: &CompiledModule) -> Result<bool> {
        match op {
            Opcode::Push(value) => {
                self.stack.push(value.clone());
            }
            Opcode::Pop => {
                self.stack.pop();
            }
            Opcode::Dup => {
                if let Some(top) = self.stack.last() {
                    self.stack.push(top.clone());
                }
            }
            Opcode::LoadLocal(idx) => {
                let base = if let Some(&(_, base, _)) = self.call_stack.last() {
                    base
                } else {
                    0
                };
                let value = self.locals.get(base + *idx as usize)
                    .cloned()
                    .unwrap_or(Value::Unit);
                self.stack.push(value);
            }
            Opcode::StoreLocal(idx) => {
                let base = if let Some(&(_, base, _)) = self.call_stack.last() {
                    base
                } else {
                    0
                };
                let value = self.stack.pop().unwrap_or(Value::Unit);
                let target = base + *idx as usize;
                while self.locals.len() <= target {
                    self.locals.push(Value::Unit);
                }
                self.locals[target] = value;
            }
            Opcode::LoadGlobal(idx) => {
                let value = self.globals.get(*idx as usize)
                    .cloned()
                    .unwrap_or(Value::Unit);
                self.stack.push(value);
            }
            Opcode::StoreGlobal(idx) => {
                let value = self.stack.pop().unwrap_or(Value::Unit);
                while self.globals.len() <= *idx as usize {
                    self.globals.push(Value::Unit);
                }
                self.globals[*idx as usize] = value;
            }
            Opcode::Add => {
                let b = self.stack.pop().unwrap_or(Value::Int(0));
                let a = self.stack.pop().unwrap_or(Value::Int(0));
                self.stack.push(self.add_values(&a, &b)?);
            }
            Opcode::Neg => {
                let a = self.stack.pop().unwrap_or(Value::Int(0));
                self.stack.push(self.negate_value(&a)?);
            }
            Opcode::Eq => {
                let b = self.stack.pop().unwrap_or(Value::Unit);
                let a = self.stack.pop().unwrap_or(Value::Unit);
                self.stack.push(Value::Bool(a == b));
            }
            Opcode::Ne => {
                let b = self.stack.pop().unwrap_or(Value::Unit);
                let a = self.stack.pop().unwrap_or(Value::Unit);
                self.stack.push(Value::Bool(a != b));
            }
            Opcode::Lt => {
                let b = self.stack.pop().unwrap_or(Value::Int(0));
                let a = self.stack.pop().unwrap_or(Value::Int(0));
                self.stack.push(Value::Bool(self.compare_lt(&a, &b)));
            }
            Opcode::Le => {
                let b = self.stack.pop().unwrap_or(Value::Int(0));
                let a = self.stack.pop().unwrap_or(Value::Int(0));
                self.stack.push(Value::Bool(self.compare_lt(&a, &b) || a == b));
            }
            Opcode::Gt => {
                let b = self.stack.pop().unwrap_or(Value::Int(0));
                let a = self.stack.pop().unwrap_or(Value::Int(0));
                self.stack.push(Value::Bool(self.compare_lt(&b, &a)));
            }
            Opcode::Ge => {
                let b = self.stack.pop().unwrap_or(Value::Int(0));
                let a = self.stack.pop().unwrap_or(Value::Int(0));
                self.stack.push(Value::Bool(self.compare_lt(&b, &a) || a == b));
            }
            Opcode::And => {
                let b = self.stack.pop().unwrap_or(Value::Bool(false));
                let a = self.stack.pop().unwrap_or(Value::Bool(false));
                self.stack.push(Value::Bool(self.is_truthy(&a) && self.is_truthy(&b)));
            }
            Opcode::Or => {
                let b = self.stack.pop().unwrap_or(Value::Bool(false));
                let a = self.stack.pop().unwrap_or(Value::Bool(false));
                self.stack.push(Value::Bool(self.is_truthy(&a) || self.is_truthy(&b)));
            }
            Opcode::Not => {
                let a = self.stack.pop().unwrap_or(Value::Bool(true));
                self.stack.push(Value::Bool(!self.is_truthy(&a)));
            }
            Opcode::Jump(addr) => {
                self.ip = *addr as usize;
                return Ok(true);
            }
            Opcode::JumpIfFalse(addr) => {
                let cond = self.stack.pop().unwrap_or(Value::Bool(false));
                if !self.is_truthy(&cond) {
                    self.ip = *addr as usize;
                    return Ok(true);
                }
            }
            Opcode::JumpIfTrue(addr) => {
                let cond = self.stack.pop().unwrap_or(Value::Bool(false));
                if self.is_truthy(&cond) {
                    self.ip = *addr as usize;
                    return Ok(true);
                }
            }
            Opcode::Call(func_idx) => {
                let func = &module.functions[*func_idx as usize];
                let locals_base = self.locals.len();

                // Pop arguments and push as locals
                let mut args = vec![];
                for _ in 0..func.arity {
                    args.push(self.stack.pop().unwrap_or(Value::Unit));
                }
                args.reverse();

                // Push call frame
                self.call_stack.push((self.ip, locals_base, func.locals));

                // Set up locals with arguments
                for arg in args {
                    self.locals.push(arg);
                }
                // Pad remaining locals
                for _ in func.arity..func.locals {
                    self.locals.push(Value::Unit);
                }

                // Execute function code
                let mut func_ip = 0;
                while func_ip < func.code.len() {
                    match &func.code[func_ip] {
                        Opcode::Return => {
                            break;
                        }
                        other => {
                            self.execute_instruction(other, module)?;
                        }
                    }
                    func_ip += 1;
                }

                // Pop call frame
                if let Some((_, base, count)) = self.call_stack.pop() {
                    self.locals.truncate(base);
                }
            }
            Opcode::Return => {
                return Ok(false);
            }
            Opcode::Print => {
                let value = self.stack.pop().unwrap_or(Value::Unit);
                println!("{}", value);
            }
            Opcode::MakeList(n) => {
                let mut items = vec![];
                for _ in 0..*n {
                    items.push(self.stack.pop().unwrap_or(Value::Unit));
                }
                items.reverse();
                self.stack.push(Value::List(items));
            }
            Opcode::MakeTuple(n) => {
                let mut items = vec![];
                for _ in 0..*n {
                    items.push(self.stack.pop().unwrap_or(Value::Unit));
                }
                items.reverse();
                self.stack.push(Value::Tuple(items));
            }
            Opcode::BeginReverse | Opcode::EndReverse => {
                // Markers for reversible sections
            }
            Opcode::Halt => {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn add_values(&self, a: &Value, b: &Value) -> Result<Value> {
        match (a, b) {
            (Value::Int(x), Value::Int(y)) => Ok(Value::Int(x + y)),
            (Value::Float(x), Value::Float(y)) => Ok(Value::Float(x + y)),
            (Value::Int(x), Value::Float(y)) => Ok(Value::Float(*x as f64 + y)),
            (Value::Float(x), Value::Int(y)) => Ok(Value::Float(x + *y as f64)),
            (Value::Rational(n1, d1), Value::Rational(n2, d2)) => {
                Ok(Value::Rational(n1 * d2 + n2 * d1, d1 * d2))
            }
            (Value::Complex(r1, i1), Value::Complex(r2, i2)) => {
                Ok(Value::Complex(r1 + r2, i1 + i2))
            }
            _ => Err(JtvError::TypeError(format!("Cannot add {:?} and {:?}", a, b))),
        }
    }

    fn negate_value(&self, a: &Value) -> Result<Value> {
        match a {
            Value::Int(x) => Ok(Value::Int(-x)),
            Value::Float(x) => Ok(Value::Float(-x)),
            Value::Rational(n, d) => Ok(Value::Rational(-n, *d)),
            Value::Complex(r, i) => Ok(Value::Complex(-r, -i)),
            _ => Err(JtvError::TypeError(format!("Cannot negate {:?}", a))),
        }
    }

    fn compare_lt(&self, a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Int(x), Value::Int(y)) => x < y,
            (Value::Float(x), Value::Float(y)) => x < y,
            (Value::Int(x), Value::Float(y)) => (*x as f64) < *y,
            (Value::Float(x), Value::Int(y)) => *x < (*y as f64),
            _ => false,
        }
    }

    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Bool(b) => *b,
            Value::Int(n) => *n != 0,
            Value::Float(f) => *f != 0.0,
            Value::Unit => false,
            Value::List(items) => !items.is_empty(),
            _ => true,
        }
    }
}

impl Default for BytecodeVM {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_program;

    #[test]
    fn test_compile_simple() {
        let code = "x = 5 + 3";
        let program = parse_program(code).unwrap();
        let mut compiler = BytecodeCompiler::new();
        let module = compiler.compile(&program).unwrap();

        let mut vm = BytecodeVM::new();
        vm.execute(&module).unwrap();
    }

    #[test]
    fn test_compile_function() {
        let code = r#"
            fn add(a: Int, b: Int): Int {
                return a + b
            }
            result = add(5, 3)
        "#;
        let program = parse_program(code).unwrap();
        let mut compiler = BytecodeCompiler::new();
        let module = compiler.compile(&program).unwrap();

        assert!(module.functions.len() > 0);
    }

    #[test]
    fn test_compile_if() {
        let code = r#"
            x = 5
            if x > 0 {
                y = 1
            }
        "#;
        let program = parse_program(code).unwrap();
        let mut compiler = BytecodeCompiler::new();
        let module = compiler.compile(&program).unwrap();

        let mut vm = BytecodeVM::new();
        vm.execute(&module).unwrap();
    }

    #[test]
    fn test_compile_loop() {
        let code = r#"
            sum = 0
            for i in 1..5 {
                sum = sum + i
            }
        "#;
        let program = parse_program(code).unwrap();
        let mut compiler = BytecodeCompiler::new();
        let module = compiler.compile(&program).unwrap();

        let mut vm = BytecodeVM::new();
        vm.execute(&module).unwrap();
    }
}
