// Type checker for Julia the Viper
// Implements static type checking for the 7 number systems and compound types

use crate::ast::*;
use crate::error::{JtvError, Result};
use std::collections::HashMap;

/// The 7 number systems plus compound types
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Rational,
    Complex,
    Hex,
    Binary,
    Symbolic,
    Bool,
    String,
    Unit,
    List(Box<Type>),
    Tuple(Vec<Type>),
    Function(Vec<Type>, Box<Type>),
    Any, // For type inference placeholder
}

impl Type {
    /// Check if this type can be coerced to another
    pub fn coercible_to(&self, target: &Type) -> bool {
        if self == target {
            return true;
        }
        matches!(
            (self, target),
            // Int can be promoted
            (Type::Int, Type::Float)
                | (Type::Int, Type::Rational)
                | (Type::Int, Type::Complex)
                // Hex and Binary are int representations
                | (Type::Hex, Type::Int)
                | (Type::Binary, Type::Int)
                // Float can be promoted to Complex
                | (Type::Float, Type::Complex)
                // Any matches everything (for inference)
                | (Type::Any, _)
                | (_, Type::Any)
        )
    }

    /// Get the result type of adding two types
    pub fn add_result(&self, other: &Type) -> Option<Type> {
        match (self, other) {
            // Same types
            (Type::Int, Type::Int) => Some(Type::Int),
            (Type::Float, Type::Float) => Some(Type::Float),
            (Type::Rational, Type::Rational) => Some(Type::Rational),
            (Type::Complex, Type::Complex) => Some(Type::Complex),
            (Type::Hex, Type::Hex) => Some(Type::Hex),
            (Type::Binary, Type::Binary) => Some(Type::Binary),
            (Type::Symbolic, Type::Symbolic) => Some(Type::Symbolic),
            (Type::String, Type::String) => Some(Type::String),

            // Coercions
            (Type::Int, Type::Float) | (Type::Float, Type::Int) => Some(Type::Float),
            (Type::Int, Type::Rational) | (Type::Rational, Type::Int) => Some(Type::Rational),
            (Type::Int, Type::Complex) | (Type::Complex, Type::Int) => Some(Type::Complex),
            (Type::Float, Type::Complex) | (Type::Complex, Type::Float) => Some(Type::Complex),
            (Type::Hex, Type::Int) | (Type::Int, Type::Hex) => Some(Type::Int),
            (Type::Binary, Type::Int) | (Type::Int, Type::Binary) => Some(Type::Int),

            // Any type
            (Type::Any, t) | (t, Type::Any) => Some(t.clone()),

            _ => None,
        }
    }

    /// Get the result type of negating a type
    pub fn negate_result(&self) -> Option<Type> {
        match self {
            Type::Int => Some(Type::Int),
            Type::Float => Some(Type::Float),
            Type::Rational => Some(Type::Rational),
            Type::Complex => Some(Type::Complex),
            Type::Hex => Some(Type::Hex),
            Type::Binary => Some(Type::Binary),
            Type::Symbolic => Some(Type::Symbolic),
            Type::Any => Some(Type::Any),
            _ => None,
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Int => write!(f, "Int"),
            Type::Float => write!(f, "Float"),
            Type::Rational => write!(f, "Rational"),
            Type::Complex => write!(f, "Complex"),
            Type::Hex => write!(f, "Hex"),
            Type::Binary => write!(f, "Binary"),
            Type::Symbolic => write!(f, "Symbolic"),
            Type::Bool => write!(f, "Bool"),
            Type::String => write!(f, "String"),
            Type::Unit => write!(f, "()"),
            Type::List(t) => write!(f, "List<{}>", t),
            Type::Tuple(ts) => {
                write!(f, "(")?;
                for (i, t) in ts.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", t)?;
                }
                write!(f, ")")
            }
            Type::Function(params, ret) => {
                write!(f, "Fn(")?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, ") -> {}", ret)
            }
            Type::Any => write!(f, "Any"),
        }
    }
}

/// Type environment: maps variable names to types
#[derive(Debug, Clone)]
pub struct TypeEnv {
    vars: HashMap<String, Type>,
    funcs: HashMap<String, (Vec<Type>, Type, Purity)>, // (params, return, purity)
}

impl TypeEnv {
    pub fn new() -> Self {
        TypeEnv {
            vars: HashMap::new(),
            funcs: HashMap::new(),
        }
    }

    pub fn get_var(&self, name: &str) -> Option<&Type> {
        self.vars.get(name)
    }

    pub fn set_var(&mut self, name: String, ty: Type) {
        self.vars.insert(name, ty);
    }

    pub fn get_func(&self, name: &str) -> Option<&(Vec<Type>, Type, Purity)> {
        self.funcs.get(name)
    }

    pub fn set_func(&mut self, name: String, params: Vec<Type>, ret: Type, purity: Purity) {
        self.funcs.insert(name, (params, ret, purity));
    }
}

impl Default for TypeEnv {
    fn default() -> Self {
        Self::new()
    }
}

/// Type checker for JtV programs
pub struct TypeChecker {
    env: TypeEnv,
    errors: Vec<JtvError>,
    /// Track type constraints for inference
    constraints: Vec<TypeConstraint>,
    /// Current function return type (for return statement checking)
    expected_return: Option<Type>,
}

/// Type constraint for inference
#[derive(Debug, Clone)]
pub struct TypeConstraint {
    pub lhs: Type,
    pub rhs: Type,
    pub context: String,
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            env: TypeEnv::new(),
            errors: vec![],
            constraints: vec![],
            expected_return: None,
        }
    }

    /// Add a type constraint for inference
    fn add_constraint(&mut self, lhs: Type, rhs: Type, context: &str) {
        self.constraints.push(TypeConstraint {
            lhs,
            rhs,
            context: context.to_string(),
        });
    }

    /// Unify two types, returning the unified type or None if incompatible
    fn unify(&self, t1: &Type, t2: &Type) -> Option<Type> {
        if t1 == t2 {
            return Some(t1.clone());
        }

        match (t1, t2) {
            // Any unifies with anything
            (Type::Any, t) | (t, Type::Any) => Some(t.clone()),

            // Coercible types unify to the wider type
            (Type::Int, Type::Float) | (Type::Float, Type::Int) => Some(Type::Float),
            (Type::Int, Type::Rational) | (Type::Rational, Type::Int) => Some(Type::Rational),
            (Type::Int, Type::Complex) | (Type::Complex, Type::Int) => Some(Type::Complex),
            (Type::Float, Type::Complex) | (Type::Complex, Type::Float) => Some(Type::Complex),
            (Type::Hex, Type::Int) | (Type::Int, Type::Hex) => Some(Type::Int),
            (Type::Binary, Type::Int) | (Type::Int, Type::Binary) => Some(Type::Int),

            // Lists unify if their element types unify
            (Type::List(a), Type::List(b)) => {
                self.unify(a, b).map(|t| Type::List(Box::new(t)))
            }

            // Tuples unify if all elements unify
            (Type::Tuple(a), Type::Tuple(b)) if a.len() == b.len() => {
                let unified: Option<Vec<Type>> = a
                    .iter()
                    .zip(b.iter())
                    .map(|(t1, t2)| self.unify(t1, t2))
                    .collect();
                unified.map(Type::Tuple)
            }

            // Functions unify if params and return types unify
            (Type::Function(p1, r1), Type::Function(p2, r2)) if p1.len() == p2.len() => {
                let unified_params: Option<Vec<Type>> = p1
                    .iter()
                    .zip(p2.iter())
                    .map(|(t1, t2)| self.unify(t1, t2))
                    .collect();
                let unified_ret = self.unify(r1, r2);
                match (unified_params, unified_ret) {
                    (Some(params), Some(ret)) => Some(Type::Function(params, Box::new(ret))),
                    _ => None,
                }
            }

            _ => None,
        }
    }

    /// Get a helpful suggestion for type mismatches
    fn type_suggestion(&self, expected: &Type, got: &Type) -> String {
        match (expected, got) {
            (Type::Int, Type::Float) => "Consider using a rational instead of float for exact arithmetic".to_string(),
            (Type::Float, Type::Int) => "You can assign an Int to a Float variable".to_string(),
            (Type::Bool, t) => format!("Use a comparison expression to convert {} to Bool", t),
            (Type::List(_), Type::Tuple(_)) => "Lists use [...], tuples use (...)".to_string(),
            _ => format!("Cannot convert {} to {}", got, expected),
        }
    }

    /// Check a complete program
    pub fn check_program(&mut self, program: &Program) -> Result<()> {
        // First pass: collect function signatures
        for stmt in &program.statements {
            if let TopLevel::Function(func) = stmt {
                self.register_function(func)?;
            }
        }

        // Second pass: type check everything
        for stmt in &program.statements {
            self.check_top_level(stmt)?;
        }

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors[0].clone())
        }
    }

    fn register_function(&mut self, func: &FunctionDecl) -> Result<()> {
        let params: Vec<Type> = func
            .params
            .iter()
            .map(|p| self.annotation_to_type(&p.type_annotation))
            .collect();

        let ret = func
            .return_type
            .as_ref()
            .map(|t| self.annotation_to_type(&Some(t.clone())))
            .unwrap_or(Type::Unit);

        self.env.set_func(func.name.clone(), params, ret, func.purity.clone());
        Ok(())
    }

    fn annotation_to_type(&self, ann: &Option<TypeAnnotation>) -> Type {
        match ann {
            None => Type::Any,
            Some(TypeAnnotation::Basic(basic)) => match basic {
                BasicType::Int => Type::Int,
                BasicType::Float => Type::Float,
                BasicType::Rational => Type::Rational,
                BasicType::Complex => Type::Complex,
                BasicType::Hex => Type::Hex,
                BasicType::Binary => Type::Binary,
                BasicType::Symbolic => Type::Symbolic,
                BasicType::Bool => Type::Bool,
                BasicType::String => Type::String,
            },
            Some(TypeAnnotation::List(inner)) => {
                Type::List(Box::new(self.annotation_to_type(&Some(*inner.clone()))))
            }
            Some(TypeAnnotation::Tuple(types)) => {
                Type::Tuple(types.iter().map(|t| self.annotation_to_type(&Some(t.clone()))).collect())
            }
            Some(TypeAnnotation::Function(params, ret)) => {
                let param_types: Vec<Type> = params
                    .iter()
                    .map(|t| self.annotation_to_type(&Some(t.clone())))
                    .collect();
                Type::Function(
                    param_types,
                    Box::new(self.annotation_to_type(&Some(*ret.clone()))),
                )
            }
        }
    }

    fn check_top_level(&mut self, top_level: &TopLevel) -> Result<()> {
        match top_level {
            TopLevel::Module(module) => {
                for stmt in &module.body {
                    self.check_top_level(stmt)?;
                }
                Ok(())
            }
            TopLevel::Import(_) => Ok(()), // Imports handled separately
            TopLevel::Function(func) => self.check_function(func),
            TopLevel::Control(stmt) => {
                self.check_control_stmt(stmt)?;
                Ok(())
            }
        }
    }

    fn check_function(&mut self, func: &FunctionDecl) -> Result<()> {
        // Create new scope with parameters
        let old_env = self.env.clone();
        let old_expected_return = self.expected_return.clone();

        // Set expected return type
        let expected_ret = func
            .return_type
            .as_ref()
            .map(|t| self.annotation_to_type(&Some(t.clone())))
            .unwrap_or(Type::Unit);
        self.expected_return = Some(expected_ret.clone());

        for param in &func.params {
            let ty = self.annotation_to_type(&param.type_annotation);
            self.env.set_var(param.name.clone(), ty);
        }

        // Check function body and infer return type
        let mut inferred_return = Type::Unit;
        for stmt in &func.body {
            if let Some(ret_ty) = self.check_control_stmt_with_return(stmt)? {
                inferred_return = ret_ty;
            }
        }

        // Verify inferred return matches declared return
        if expected_ret != Type::Unit && inferred_return != Type::Any {
            if self.unify(&expected_ret, &inferred_return).is_none() {
                return Err(JtvError::TypeError(format!(
                    "Function '{}' declares return type {} but returns {}. {}",
                    func.name,
                    expected_ret,
                    inferred_return,
                    self.type_suggestion(&expected_ret, &inferred_return)
                )));
            }
        }

        // Restore environment
        self.env = old_env;
        self.expected_return = old_expected_return;
        Ok(())
    }

    /// Check a control statement and return the type if it's a return statement
    fn check_control_stmt_with_return(&mut self, stmt: &ControlStmt) -> Result<Option<Type>> {
        match stmt {
            ControlStmt::Return(expr) => {
                let ret_ty = if let Some(e) = expr {
                    self.infer_data_expr(e)?
                } else {
                    Type::Unit
                };

                // Check against expected return type
                if let Some(expected) = &self.expected_return {
                    if self.unify(expected, &ret_ty).is_none() {
                        return Err(JtvError::TypeError(format!(
                            "Return type mismatch: expected {}, got {}. {}",
                            expected,
                            ret_ty,
                            self.type_suggestion(expected, &ret_ty)
                        )));
                    }
                }

                Ok(Some(ret_ty))
            }
            ControlStmt::If(if_stmt) => {
                self.infer_control_expr(&if_stmt.condition)?;
                let mut ret_ty = None;

                for s in &if_stmt.then_branch {
                    if let Some(ty) = self.check_control_stmt_with_return(s)? {
                        ret_ty = Some(ty);
                    }
                }
                if let Some(else_branch) = &if_stmt.else_branch {
                    for s in else_branch {
                        if let Some(ty) = self.check_control_stmt_with_return(s)? {
                            ret_ty = Some(ty);
                        }
                    }
                }
                Ok(ret_ty)
            }
            ControlStmt::Block(stmts) => {
                let mut ret_ty = None;
                for s in stmts {
                    if let Some(ty) = self.check_control_stmt_with_return(s)? {
                        ret_ty = Some(ty);
                    }
                }
                Ok(ret_ty)
            }
            _ => {
                self.check_control_stmt(stmt)?;
                Ok(None)
            }
        }
    }

    fn check_control_stmt(&mut self, stmt: &ControlStmt) -> Result<()> {
        match stmt {
            ControlStmt::Assignment(assign) => {
                let ty = match &assign.value {
                    Expr::Data(expr) => self.infer_data_expr(expr)?,
                    Expr::Control(expr) => self.infer_control_expr(expr)?,
                };
                self.env.set_var(assign.target.clone(), ty);
                Ok(())
            }
            ControlStmt::If(if_stmt) => {
                // Condition must be evaluable (now supports comparisons)
                self.infer_control_expr(&if_stmt.condition)?;

                for stmt in &if_stmt.then_branch {
                    self.check_control_stmt(stmt)?;
                }
                if let Some(else_branch) = &if_stmt.else_branch {
                    for stmt in else_branch {
                        self.check_control_stmt(stmt)?;
                    }
                }
                Ok(())
            }
            ControlStmt::While(while_stmt) => {
                self.infer_control_expr(&while_stmt.condition)?;
                for stmt in &while_stmt.body {
                    self.check_control_stmt(stmt)?;
                }
                Ok(())
            }
            ControlStmt::For(for_stmt) => {
                // Range endpoints must be integers
                let start_ty = self.infer_data_expr(&for_stmt.range.start)?;
                let end_ty = self.infer_data_expr(&for_stmt.range.end)?;

                if !start_ty.coercible_to(&Type::Int) {
                    return Err(JtvError::TypeError(format!(
                        "Range start must be Int, got {}",
                        start_ty
                    )));
                }
                if !end_ty.coercible_to(&Type::Int) {
                    return Err(JtvError::TypeError(format!(
                        "Range end must be Int, got {}",
                        end_ty
                    )));
                }

                // Loop variable is Int
                self.env.set_var(for_stmt.variable.clone(), Type::Int);

                for stmt in &for_stmt.body {
                    self.check_control_stmt(stmt)?;
                }
                Ok(())
            }
            ControlStmt::Return(expr) => {
                if let Some(e) = expr {
                    self.infer_data_expr(e)?;
                }
                Ok(())
            }
            ControlStmt::Print(exprs) => {
                for expr in exprs {
                    self.infer_data_expr(expr)?;
                }
                Ok(())
            }
            ControlStmt::ReverseBlock(block) => {
                for stmt in &block.body {
                    self.check_reversible_stmt(stmt)?;
                }
                Ok(())
            }
            ControlStmt::Block(stmts) => {
                for stmt in stmts {
                    self.check_control_stmt(stmt)?;
                }
                Ok(())
            }
        }
    }

    fn check_reversible_stmt(&mut self, stmt: &ReversibleStmt) -> Result<()> {
        match stmt {
            ReversibleStmt::AddAssign(target, expr) | ReversibleStmt::SubAssign(target, expr) => {
                let expr_ty = self.infer_data_expr(expr)?;
                let target_ty = self.env.get_var(target).cloned().unwrap_or(Type::Any);

                if target_ty.add_result(&expr_ty).is_none() {
                    return Err(JtvError::TypeError(format!(
                        "Cannot add {} to {}",
                        expr_ty, target_ty
                    )));
                }
                Ok(())
            }
            ReversibleStmt::If(if_stmt) => {
                self.infer_control_expr(&if_stmt.condition)?;
                for stmt in &if_stmt.then_branch {
                    self.check_control_stmt(stmt)?;
                }
                if let Some(else_branch) = &if_stmt.else_branch {
                    for stmt in else_branch {
                        self.check_control_stmt(stmt)?;
                    }
                }
                Ok(())
            }
        }
    }

    /// Infer the type of a Data expression
    pub fn infer_data_expr(&self, expr: &DataExpr) -> Result<Type> {
        match expr {
            DataExpr::Number(num) => Ok(self.number_type(num)),
            DataExpr::Identifier(name) => self
                .env
                .get_var(name)
                .cloned()
                .ok_or_else(|| JtvError::UndefinedVariable(name.clone())),
            DataExpr::Add(left, right) => {
                let left_ty = self.infer_data_expr(left)?;
                let right_ty = self.infer_data_expr(right)?;

                left_ty.add_result(&right_ty).ok_or_else(|| {
                    JtvError::TypeError(format!("Cannot add {} and {}", left_ty, right_ty))
                })
            }
            DataExpr::Negate(inner) => {
                let inner_ty = self.infer_data_expr(inner)?;
                inner_ty.negate_result().ok_or_else(|| {
                    JtvError::TypeError(format!("Cannot negate {}", inner_ty))
                })
            }
            DataExpr::FunctionCall(call) => {
                if let Some((param_types, ret_ty, _)) = self.env.get_func(&call.name) {
                    // Check argument count
                    if call.args.len() != param_types.len() {
                        return Err(JtvError::ArityMismatch {
                            expected: param_types.len(),
                            got: call.args.len(),
                        });
                    }

                    // Check argument types
                    for (arg, expected_ty) in call.args.iter().zip(param_types.iter()) {
                        let arg_ty = self.infer_data_expr(arg)?;
                        if !arg_ty.coercible_to(expected_ty) {
                            return Err(JtvError::TypeError(format!(
                                "Expected {}, got {}",
                                expected_ty, arg_ty
                            )));
                        }
                    }

                    Ok(ret_ty.clone())
                } else {
                    Err(JtvError::UndefinedFunction(call.name.clone()))
                }
            }
            DataExpr::List(elements) => {
                if elements.is_empty() {
                    Ok(Type::List(Box::new(Type::Any)))
                } else {
                    let first_ty = self.infer_data_expr(&elements[0])?;
                    // Check all elements have compatible types
                    for elem in &elements[1..] {
                        let elem_ty = self.infer_data_expr(elem)?;
                        if !elem_ty.coercible_to(&first_ty) && !first_ty.coercible_to(&elem_ty) {
                            return Err(JtvError::TypeError(format!(
                                "List elements must have consistent types: {} vs {}",
                                first_ty, elem_ty
                            )));
                        }
                    }
                    Ok(Type::List(Box::new(first_ty)))
                }
            }
            DataExpr::Tuple(elements) => {
                let types: Result<Vec<Type>> =
                    elements.iter().map(|e| self.infer_data_expr(e)).collect();
                Ok(Type::Tuple(types?))
            }
        }
    }

    fn infer_control_expr(&self, expr: &ControlExpr) -> Result<Type> {
        match expr {
            ControlExpr::Data(data) => self.infer_data_expr(data),
            ControlExpr::Comparison(left, _, right) => {
                self.infer_data_expr(left)?;
                self.infer_data_expr(right)?;
                Ok(Type::Bool)
            }
            ControlExpr::Logical(left, _, right) => {
                self.infer_control_expr(left)?;
                self.infer_control_expr(right)?;
                Ok(Type::Bool)
            }
            ControlExpr::Not(inner) => {
                self.infer_control_expr(inner)?;
                Ok(Type::Bool)
            }
        }
    }

    fn number_type(&self, num: &Number) -> Type {
        match num {
            Number::Int(_) => Type::Int,
            Number::Float(_) => Type::Float,
            Number::Rational(_, _) => Type::Rational,
            Number::Complex(_, _) => Type::Complex,
            Number::Hex(_) => Type::Hex,
            Number::Binary(_) => Type::Binary,
            Number::Symbolic(_) => Type::Symbolic,
        }
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_program;

    #[test]
    fn test_simple_type_check() {
        let code = "x = 5 + 3";
        let program = parse_program(code).unwrap();
        let mut checker = TypeChecker::new();
        assert!(checker.check_program(&program).is_ok());
    }

    #[test]
    fn test_different_types() {
        // Test that different variables can have different number types
        let code = r#"
            x = 5
            y = 3.14
        "#;
        let program = parse_program(code).unwrap();
        let mut checker = TypeChecker::new();
        // This should succeed - different variables can have different types
        assert!(checker.check_program(&program).is_ok());
    }

    #[test]
    fn test_function_type_check() {
        let code = r#"
            fn add(a: Int, b: Int): Int {
                return a + b
            }
        "#;
        let program = parse_program(code).unwrap();
        let mut checker = TypeChecker::new();
        assert!(checker.check_program(&program).is_ok());
    }

    #[test]
    fn test_coercion() {
        let code = r#"
            x = 5
            y = 3.14
            z = x + y
        "#;
        let program = parse_program(code).unwrap();
        let mut checker = TypeChecker::new();
        // Int + Float should coerce to Float
        assert!(checker.check_program(&program).is_ok());
    }

    #[test]
    fn test_return_type_check() {
        let code = r#"
            fn double(x: Int): Int {
                return x + x
            }
        "#;
        let program = parse_program(code).unwrap();
        let mut checker = TypeChecker::new();
        assert!(checker.check_program(&program).is_ok());
    }

    #[test]
    fn test_function_call_type_check() {
        let code = r#"
            fn add(a: Int, b: Int): Int {
                return a + b
            }
            result = add(5, 3)
        "#;
        let program = parse_program(code).unwrap();
        let mut checker = TypeChecker::new();
        assert!(checker.check_program(&program).is_ok());
    }

    #[test]
    fn test_list_type_inference() {
        let code = r#"
            numbers = [1, 2, 3, 4, 5]
        "#;
        let program = parse_program(code).unwrap();
        let mut checker = TypeChecker::new();
        assert!(checker.check_program(&program).is_ok());
    }

    #[test]
    fn test_tuple_type_inference() {
        let code = r#"
            point = (10, 20, 30)
        "#;
        let program = parse_program(code).unwrap();
        let mut checker = TypeChecker::new();
        assert!(checker.check_program(&program).is_ok());
    }

    #[test]
    fn test_comparison_type_bool() {
        let code = r#"
            x = 5
            if x > 0 {
                y = 1
            }
        "#;
        let program = parse_program(code).unwrap();
        let mut checker = TypeChecker::new();
        assert!(checker.check_program(&program).is_ok());
    }

    #[test]
    fn test_logical_operators_type() {
        let code = r#"
            x = 5
            y = 10
            if x > 0 && y > 0 {
                z = 1
            }
        "#;
        let program = parse_program(code).unwrap();
        let mut checker = TypeChecker::new();
        assert!(checker.check_program(&program).is_ok());
    }

    #[test]
    fn test_undefined_variable_error() {
        let code = r#"
            y = undefined_var + 1
        "#;
        let program = parse_program(code).unwrap();
        let mut checker = TypeChecker::new();
        assert!(checker.check_program(&program).is_err());
    }

    #[test]
    fn test_undefined_function_error() {
        let code = r#"
            result = unknown_func(5)
        "#;
        let program = parse_program(code).unwrap();
        let mut checker = TypeChecker::new();
        assert!(checker.check_program(&program).is_err());
    }

    #[test]
    fn test_arity_mismatch_error() {
        let code = r#"
            fn add(a: Int, b: Int): Int {
                return a + b
            }
            result = add(5)
        "#;
        let program = parse_program(code).unwrap();
        let mut checker = TypeChecker::new();
        assert!(checker.check_program(&program).is_err());
    }
}
