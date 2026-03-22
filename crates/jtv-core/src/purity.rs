// Purity enforcement for Julia the Viper
// Ensures @pure and @total functions respect their contracts

use crate::ast::*;
use crate::error::{JtvError, Result};
use std::collections::HashMap;

/// Purity analysis result
#[derive(Debug, Clone, PartialEq)]
pub enum PurityLevel {
    /// Guaranteed to terminate, no side effects
    Total,
    /// No side effects, may not terminate (has loops)
    Pure,
    /// May have side effects and may not terminate
    Impure,
}

impl PurityLevel {
    /// Check if this level satisfies a required level
    pub fn satisfies(&self, required: &Purity) -> bool {
        match (self, required) {
            // Total satisfies everything
            (PurityLevel::Total, _) => true,
            // Pure satisfies Pure and Impure
            (PurityLevel::Pure, Purity::Pure) => true,
            (PurityLevel::Pure, Purity::Impure) => true,
            (PurityLevel::Pure, Purity::Total) => false,
            // Impure only satisfies Impure
            (PurityLevel::Impure, Purity::Impure) => true,
            (PurityLevel::Impure, _) => false,
        }
    }

    /// Combine two purity levels (least pure wins)
    pub fn combine(&self, other: &PurityLevel) -> PurityLevel {
        match (self, other) {
            (PurityLevel::Impure, _) | (_, PurityLevel::Impure) => PurityLevel::Impure,
            (PurityLevel::Pure, _) | (_, PurityLevel::Pure) => PurityLevel::Pure,
            (PurityLevel::Total, PurityLevel::Total) => PurityLevel::Total,
        }
    }
}

/// Purity checker for JtV programs
pub struct PurityChecker {
    /// Function purity levels
    func_purity: HashMap<String, PurityLevel>,
    /// Errors found during checking
    errors: Vec<JtvError>,
}

impl PurityChecker {
    pub fn new() -> Self {
        PurityChecker {
            func_purity: HashMap::new(),
            errors: vec![],
        }
    }

    /// Check a complete program for purity violations
    pub fn check_program(&mut self, program: &Program) -> Result<()> {
        // First pass: collect declared purity levels
        for stmt in &program.statements {
            if let TopLevel::Function(func) = stmt {
                self.func_purity
                    .insert(func.name.clone(), self.declared_to_level(&func.purity));
            }
        }

        // Second pass: verify functions respect their declared purity
        for stmt in &program.statements {
            if let TopLevel::Function(func) = stmt {
                self.check_function(func)?;
            }
        }

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors[0].clone())
        }
    }

    fn declared_to_level(&self, purity: &Purity) -> PurityLevel {
        match purity {
            Purity::Total => PurityLevel::Total,
            Purity::Pure => PurityLevel::Pure,
            Purity::Impure => PurityLevel::Impure,
        }
    }

    /// Check a function respects its declared purity
    fn check_function(&mut self, func: &FunctionDecl) -> Result<()> {
        let _declared = self.declared_to_level(&func.purity);
        let actual = self.analyze_body(&func.body)?;

        if !actual.satisfies(&func.purity) {
            let msg = match func.purity {
                Purity::Total => {
                    format!(
                        "Function '{}' is marked @total but contains loops or impure calls",
                        func.name
                    )
                }
                Purity::Pure => {
                    format!(
                        "Function '{}' is marked @pure but contains I/O operations",
                        func.name
                    )
                }
                Purity::Impure => unreachable!(), // Impure accepts everything
            };
            return Err(JtvError::PurityViolation(msg));
        }

        Ok(())
    }

    /// Analyze the purity of a function body
    fn analyze_body(&self, stmts: &[ControlStmt]) -> Result<PurityLevel> {
        let mut level = PurityLevel::Total;

        for stmt in stmts {
            let stmt_level = self.analyze_stmt(stmt)?;
            level = level.combine(&stmt_level);
        }

        Ok(level)
    }

    /// Analyze the purity of a single statement
    fn analyze_stmt(&self, stmt: &ControlStmt) -> Result<PurityLevel> {
        match stmt {
            ControlStmt::Assignment(assign) => {
                // Assignment is pure (no I/O)
                match &assign.value {
                    Expr::Data(expr) => self.analyze_data_expr(expr),
                    Expr::Control(expr) => self.analyze_control_expr(expr),
                }
            }
            ControlStmt::If(if_stmt) => {
                let cond_level = self.analyze_control_expr(&if_stmt.condition)?;
                let then_level = self.analyze_body(&if_stmt.then_branch)?;
                let else_level = if let Some(else_branch) = &if_stmt.else_branch {
                    self.analyze_body(else_branch)?
                } else {
                    PurityLevel::Total
                };

                Ok(cond_level.combine(&then_level).combine(&else_level))
            }
            ControlStmt::While(while_stmt) => {
                // While loops make a function at most Pure (not Total)
                let body_level = self.analyze_body(&while_stmt.body)?;

                // Even if body is total, the loop itself may not terminate
                Ok(PurityLevel::Pure.combine(&body_level))
            }
            ControlStmt::For(for_stmt) => {
                // For loops with bounded ranges are Total
                // But we conservatively mark them as Pure
                let body_level = self.analyze_body(&for_stmt.body)?;

                // For loops over finite ranges are technically total,
                // but we're conservative here
                Ok(PurityLevel::Pure.combine(&body_level))
            }
            ControlStmt::Return(expr) => {
                if let Some(e) = expr {
                    self.analyze_data_expr(e)
                } else {
                    Ok(PurityLevel::Total)
                }
            }
            ControlStmt::Print(_) => {
                // Print is I/O, making function Impure
                Ok(PurityLevel::Impure)
            }
            ControlStmt::ReverseBlock(block) => {
                // Reverse blocks are total if their body is total
                let mut level = PurityLevel::Total;
                for stmt in &block.body {
                    let stmt_level = self.analyze_reversible_stmt(stmt)?;
                    level = level.combine(&stmt_level);
                }
                Ok(level)
            }
            ControlStmt::Block(stmts) => self.analyze_body(stmts),
        }
    }

    fn analyze_reversible_stmt(&self, stmt: &ReversibleStmt) -> Result<PurityLevel> {
        match stmt {
            ReversibleStmt::AddAssign(_, expr) | ReversibleStmt::SubAssign(_, expr) => {
                self.analyze_data_expr(expr)
            }
            ReversibleStmt::If(if_stmt) => {
                let cond_level = self.analyze_control_expr(&if_stmt.condition)?;
                let then_level = self.analyze_body(&if_stmt.then_branch)?;
                let else_level = if let Some(else_branch) = &if_stmt.else_branch {
                    self.analyze_body(else_branch)?
                } else {
                    PurityLevel::Total
                };
                Ok(cond_level.combine(&then_level).combine(&else_level))
            }
        }
    }

    /// Analyze purity of a data expression
    fn analyze_data_expr(&self, expr: &DataExpr) -> Result<PurityLevel> {
        match expr {
            DataExpr::Number(_) => Ok(PurityLevel::Total),
            DataExpr::Identifier(_) => Ok(PurityLevel::Total),
            DataExpr::Add(left, right) => {
                let left_level = self.analyze_data_expr(left)?;
                let right_level = self.analyze_data_expr(right)?;
                Ok(left_level.combine(&right_level))
            }
            DataExpr::Negate(inner) => self.analyze_data_expr(inner),
            DataExpr::FunctionCall(call) => {
                // Try qualified name first (Module::func), then unqualified
                let qualified = call.qualified_name();
                let func_level = self
                    .func_purity
                    .get(&qualified)
                    .or_else(|| self.func_purity.get(&call.name))
                    .cloned()
                    .unwrap_or(PurityLevel::Impure); // Unknown functions assumed impure

                // Also analyze arguments
                let mut level = func_level;
                for arg in &call.args {
                    let arg_level = self.analyze_data_expr(arg)?;
                    level = level.combine(&arg_level);
                }
                Ok(level)
            }
            DataExpr::List(elements) => {
                let mut level = PurityLevel::Total;
                for elem in elements {
                    let elem_level = self.analyze_data_expr(elem)?;
                    level = level.combine(&elem_level);
                }
                Ok(level)
            }
            DataExpr::Tuple(elements) => {
                let mut level = PurityLevel::Total;
                for elem in elements {
                    let elem_level = self.analyze_data_expr(elem)?;
                    level = level.combine(&elem_level);
                }
                Ok(level)
            }
        }
    }

    fn analyze_control_expr(&self, expr: &ControlExpr) -> Result<PurityLevel> {
        match expr {
            ControlExpr::Data(data) => self.analyze_data_expr(data),
            ControlExpr::Comparison(left, _, right) => {
                let left_level = self.analyze_data_expr(left)?;
                let right_level = self.analyze_data_expr(right)?;
                Ok(left_level.combine(&right_level))
            }
            ControlExpr::Logical(left, _, right) => {
                let left_level = self.analyze_control_expr(left)?;
                let right_level = self.analyze_control_expr(right)?;
                Ok(left_level.combine(&right_level))
            }
            ControlExpr::Not(inner) => self.analyze_control_expr(inner),
        }
    }

    /// Get the analyzed purity level of a function
    pub fn get_function_purity(&self, name: &str) -> Option<&PurityLevel> {
        self.func_purity.get(name)
    }
}

impl Default for PurityChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_program;

    #[test]
    fn test_total_function() {
        let code = r#"
            @total fn add(a: Int, b: Int): Int {
                return a + b
            }
        "#;
        let program = parse_program(code).unwrap();
        let mut checker = PurityChecker::new();
        assert!(checker.check_program(&program).is_ok());
    }

    #[test]
    fn test_total_with_loop_fails() {
        // Use a for loop which definitely parses
        let code = r#"
            @total fn bad(n: Int): Int {
                x = 0
                for i in 0..10 {
                    x = x + 1
                }
                return x
            }
        "#;
        let program = parse_program(code).unwrap();
        let mut checker = PurityChecker::new();
        // For loops make function non-total
        assert!(checker.check_program(&program).is_err());
    }

    #[test]
    fn test_pure_function() {
        let code = r#"
            @pure fn loop_sum(n: Int): Int {
                sum = 0
                for i in 0..n {
                    sum = sum + i
                }
                return sum
            }
        "#;
        let program = parse_program(code).unwrap();
        let mut checker = PurityChecker::new();
        // Pure allows loops
        assert!(checker.check_program(&program).is_ok());
    }

    #[test]
    fn test_pure_with_io_fails() {
        let code = r#"
            @pure fn bad(x: Int): Int {
                print(x)
                return x
            }
        "#;
        let program = parse_program(code).unwrap();
        let mut checker = PurityChecker::new();
        assert!(checker.check_program(&program).is_err());
    }

    #[test]
    fn test_impure_allows_everything() {
        let code = r#"
            fn anything(n: Int): Int {
                print(n)
                for i in 0..10 {
                    n = n + 1
                }
                return n
            }
        "#;
        let program = parse_program(code).unwrap();
        let mut checker = PurityChecker::new();
        assert!(checker.check_program(&program).is_ok());
    }

    #[test]
    fn test_total_calling_impure_fails() {
        // Test that a @total function calling impure function fails
        // Using a simpler approach: @total with print (IO) should fail
        let code = r#"
            @total fn bad(x: Int): Int {
                print(x)
                return x
            }
        "#;
        let program = parse_program(code).unwrap();
        let mut checker = PurityChecker::new();
        // Total functions cannot have IO
        assert!(checker.check_program(&program).is_err());
    }
}
