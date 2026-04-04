// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
//
// Targeted tests to kill surviving mutants identified by cargo-mutants.
// These tests exist specifically to improve mutation kill rate above 80%.
//
// Primary targets:
// - Value::lt, Value::gt, Value::le, Value::ge (comparison operators)
// - Value::is_truthy (truthiness evaluation)
// - Interpreter::eval_control_stmt (control flow)
// - Formatter output correctness

use jtv_core::number::Value;
use jtv_core::parser::parse_program;
use jtv_core::formatter::Formatter;
use jtv_core::pretty::PrettyPrinter;
use jtv_core::ast::*;
use jtv_core::interpreter::Interpreter;
use num_complex::Complex64;
use num_rational::Ratio;

// ============================================================================
// Value::lt — less than comparisons across all types
// ============================================================================

#[test]
fn lt_int_true() {
    assert_eq!(Value::Int(3).lt(&Value::Int(5)).unwrap(), true);
}

#[test]
fn lt_int_false() {
    assert_eq!(Value::Int(5).lt(&Value::Int(3)).unwrap(), false);
}

#[test]
fn lt_int_equal_is_false() {
    assert_eq!(Value::Int(5).lt(&Value::Int(5)).unwrap(), false);
}

#[test]
fn lt_float_true() {
    assert_eq!(Value::Float(1.0).lt(&Value::Float(2.0)).unwrap(), true);
}

#[test]
fn lt_float_false() {
    assert_eq!(Value::Float(2.0).lt(&Value::Float(1.0)).unwrap(), false);
}

#[test]
fn lt_float_equal_is_false() {
    assert_eq!(Value::Float(1.0).lt(&Value::Float(1.0)).unwrap(), false);
}

#[test]
fn lt_rational_true() {
    let a = Value::Rational(Ratio::new(1, 3));
    let b = Value::Rational(Ratio::new(1, 2));
    assert_eq!(a.lt(&b).unwrap(), true);
}

#[test]
fn lt_rational_false() {
    let a = Value::Rational(Ratio::new(1, 2));
    let b = Value::Rational(Ratio::new(1, 3));
    assert_eq!(a.lt(&b).unwrap(), false);
}

#[test]
fn lt_hex_true() {
    assert_eq!(Value::Hex(0x0A).lt(&Value::Hex(0xFF)).unwrap(), true);
}

#[test]
fn lt_hex_false() {
    assert_eq!(Value::Hex(0xFF).lt(&Value::Hex(0x0A)).unwrap(), false);
}

#[test]
fn lt_binary_true() {
    assert_eq!(Value::Binary(0b10).lt(&Value::Binary(0b11)).unwrap(), true);
}

#[test]
fn lt_binary_false() {
    assert_eq!(Value::Binary(0b11).lt(&Value::Binary(0b10)).unwrap(), false);
}

// Cross-type lt
#[test]
fn lt_int_float_true() {
    assert_eq!(Value::Int(1).lt(&Value::Float(2.0)).unwrap(), true);
}

#[test]
fn lt_int_float_false() {
    assert_eq!(Value::Int(3).lt(&Value::Float(2.0)).unwrap(), false);
}

#[test]
fn lt_float_int_true() {
    assert_eq!(Value::Float(1.0).lt(&Value::Int(2)).unwrap(), true);
}

#[test]
fn lt_float_int_false() {
    assert_eq!(Value::Float(3.0).lt(&Value::Int(2)).unwrap(), false);
}

#[test]
fn lt_int_rational_true() {
    assert_eq!(Value::Int(0).lt(&Value::Rational(Ratio::new(1, 2))).unwrap(), true);
}

#[test]
fn lt_int_rational_false() {
    assert_eq!(Value::Int(1).lt(&Value::Rational(Ratio::new(1, 2))).unwrap(), false);
}

#[test]
fn lt_rational_int_true() {
    assert_eq!(Value::Rational(Ratio::new(1, 2)).lt(&Value::Int(1)).unwrap(), true);
}

#[test]
fn lt_rational_int_false() {
    assert_eq!(Value::Rational(Ratio::new(3, 2)).lt(&Value::Int(1)).unwrap(), false);
}

#[test]
fn lt_hex_int_true() {
    assert_eq!(Value::Hex(1).lt(&Value::Int(5)).unwrap(), true);
}

#[test]
fn lt_hex_int_false() {
    assert_eq!(Value::Hex(10).lt(&Value::Int(5)).unwrap(), false);
}

#[test]
fn lt_binary_int_true() {
    assert_eq!(Value::Binary(1).lt(&Value::Int(5)).unwrap(), true);
}

#[test]
fn lt_binary_int_false() {
    assert_eq!(Value::Binary(10).lt(&Value::Int(5)).unwrap(), false);
}

#[test]
fn lt_incompatible_types_error() {
    assert!(Value::Int(1).lt(&Value::Symbolic("x".to_string())).is_err());
}

// ============================================================================
// Value::gt — greater than comparisons
// ============================================================================

#[test]
fn gt_int_true() {
    assert_eq!(Value::Int(5).gt(&Value::Int(3)).unwrap(), true);
}

#[test]
fn gt_int_false() {
    assert_eq!(Value::Int(3).gt(&Value::Int(5)).unwrap(), false);
}

#[test]
fn gt_int_equal_is_false() {
    assert_eq!(Value::Int(5).gt(&Value::Int(5)).unwrap(), false);
}

#[test]
fn gt_float_true() {
    assert_eq!(Value::Float(2.0).gt(&Value::Float(1.0)).unwrap(), true);
}

#[test]
fn gt_float_false() {
    assert_eq!(Value::Float(1.0).gt(&Value::Float(2.0)).unwrap(), false);
}

#[test]
fn gt_rational_true() {
    let a = Value::Rational(Ratio::new(2, 3));
    let b = Value::Rational(Ratio::new(1, 3));
    assert_eq!(a.gt(&b).unwrap(), true);
}

#[test]
fn gt_rational_false() {
    let a = Value::Rational(Ratio::new(1, 3));
    let b = Value::Rational(Ratio::new(2, 3));
    assert_eq!(a.gt(&b).unwrap(), false);
}

#[test]
fn gt_hex_true() {
    assert_eq!(Value::Hex(0xFF).gt(&Value::Hex(0x0A)).unwrap(), true);
}

#[test]
fn gt_binary_true() {
    assert_eq!(Value::Binary(0b11).gt(&Value::Binary(0b10)).unwrap(), true);
}

// Cross-type gt
#[test]
fn gt_int_float_true() {
    assert_eq!(Value::Int(3).gt(&Value::Float(2.0)).unwrap(), true);
}

#[test]
fn gt_float_int_true() {
    assert_eq!(Value::Float(3.0).gt(&Value::Int(2)).unwrap(), true);
}

#[test]
fn gt_int_rational_true() {
    assert_eq!(Value::Int(1).gt(&Value::Rational(Ratio::new(1, 2))).unwrap(), true);
}

#[test]
fn gt_rational_int_true() {
    assert_eq!(Value::Rational(Ratio::new(3, 2)).gt(&Value::Int(1)).unwrap(), true);
}

#[test]
fn gt_hex_int_true() {
    assert_eq!(Value::Hex(10).gt(&Value::Int(5)).unwrap(), true);
}

#[test]
fn gt_binary_int_true() {
    assert_eq!(Value::Binary(10).gt(&Value::Int(5)).unwrap(), true);
}

// ============================================================================
// Value::le — less than or equal
// ============================================================================

#[test]
fn le_int_less() {
    assert_eq!(Value::Int(3).le(&Value::Int(5)).unwrap(), true);
}

#[test]
fn le_int_equal() {
    assert_eq!(Value::Int(5).le(&Value::Int(5)).unwrap(), true);
}

#[test]
fn le_int_greater() {
    assert_eq!(Value::Int(7).le(&Value::Int(5)).unwrap(), false);
}

#[test]
fn le_float_less() {
    assert_eq!(Value::Float(1.0).le(&Value::Float(2.0)).unwrap(), true);
}

#[test]
fn le_float_equal() {
    assert_eq!(Value::Float(2.0).le(&Value::Float(2.0)).unwrap(), true);
}

#[test]
fn le_float_greater() {
    assert_eq!(Value::Float(3.0).le(&Value::Float(2.0)).unwrap(), false);
}

#[test]
fn le_rational_less() {
    assert_eq!(Value::Rational(Ratio::new(1, 3)).le(&Value::Rational(Ratio::new(1, 2))).unwrap(), true);
}

#[test]
fn le_rational_equal() {
    assert_eq!(Value::Rational(Ratio::new(1, 2)).le(&Value::Rational(Ratio::new(1, 2))).unwrap(), true);
}

#[test]
fn le_hex_less() {
    assert_eq!(Value::Hex(5).le(&Value::Hex(10)).unwrap(), true);
}

#[test]
fn le_hex_equal() {
    assert_eq!(Value::Hex(5).le(&Value::Hex(5)).unwrap(), true);
}

#[test]
fn le_binary_less() {
    assert_eq!(Value::Binary(5).le(&Value::Binary(10)).unwrap(), true);
}

// Cross-type le
#[test]
fn le_int_float() {
    assert_eq!(Value::Int(2).le(&Value::Float(2.0)).unwrap(), true);
    assert_eq!(Value::Int(3).le(&Value::Float(2.0)).unwrap(), false);
}

#[test]
fn le_float_int() {
    assert_eq!(Value::Float(2.0).le(&Value::Int(2)).unwrap(), true);
    assert_eq!(Value::Float(3.0).le(&Value::Int(2)).unwrap(), false);
}

#[test]
fn le_int_rational() {
    assert_eq!(Value::Int(0).le(&Value::Rational(Ratio::new(1, 2))).unwrap(), true);
    assert_eq!(Value::Int(1).le(&Value::Rational(Ratio::new(1, 2))).unwrap(), false);
}

#[test]
fn le_rational_int() {
    assert_eq!(Value::Rational(Ratio::new(1, 2)).le(&Value::Int(1)).unwrap(), true);
    assert_eq!(Value::Rational(Ratio::new(3, 2)).le(&Value::Int(1)).unwrap(), false);
}

#[test]
fn le_hex_int() {
    assert_eq!(Value::Hex(5).le(&Value::Int(5)).unwrap(), true);
    assert_eq!(Value::Hex(6).le(&Value::Int(5)).unwrap(), false);
}

#[test]
fn le_binary_int() {
    assert_eq!(Value::Binary(5).le(&Value::Int(5)).unwrap(), true);
    assert_eq!(Value::Binary(6).le(&Value::Int(5)).unwrap(), false);
}

// ============================================================================
// Value::ge — greater than or equal
// ============================================================================

#[test]
fn ge_int_greater() {
    assert_eq!(Value::Int(7).ge(&Value::Int(5)).unwrap(), true);
}

#[test]
fn ge_int_equal() {
    assert_eq!(Value::Int(5).ge(&Value::Int(5)).unwrap(), true);
}

#[test]
fn ge_int_less() {
    assert_eq!(Value::Int(3).ge(&Value::Int(5)).unwrap(), false);
}

#[test]
fn ge_float_greater() {
    assert_eq!(Value::Float(3.0).ge(&Value::Float(2.0)).unwrap(), true);
}

#[test]
fn ge_float_equal() {
    assert_eq!(Value::Float(2.0).ge(&Value::Float(2.0)).unwrap(), true);
}

#[test]
fn ge_float_less() {
    assert_eq!(Value::Float(1.0).ge(&Value::Float(2.0)).unwrap(), false);
}

#[test]
fn ge_rational_greater() {
    assert_eq!(Value::Rational(Ratio::new(2, 3)).ge(&Value::Rational(Ratio::new(1, 3))).unwrap(), true);
}

#[test]
fn ge_rational_equal() {
    assert_eq!(Value::Rational(Ratio::new(1, 2)).ge(&Value::Rational(Ratio::new(1, 2))).unwrap(), true);
}

#[test]
fn ge_hex_greater() {
    assert_eq!(Value::Hex(10).ge(&Value::Hex(5)).unwrap(), true);
}

#[test]
fn ge_hex_equal() {
    assert_eq!(Value::Hex(5).ge(&Value::Hex(5)).unwrap(), true);
}

#[test]
fn ge_binary_greater() {
    assert_eq!(Value::Binary(10).ge(&Value::Binary(5)).unwrap(), true);
}

// Cross-type ge
#[test]
fn ge_int_float() {
    assert_eq!(Value::Int(2).ge(&Value::Float(2.0)).unwrap(), true);
    assert_eq!(Value::Int(1).ge(&Value::Float(2.0)).unwrap(), false);
}

#[test]
fn ge_float_int() {
    assert_eq!(Value::Float(2.0).ge(&Value::Int(2)).unwrap(), true);
    assert_eq!(Value::Float(1.0).ge(&Value::Int(2)).unwrap(), false);
}

#[test]
fn ge_int_rational() {
    assert_eq!(Value::Int(1).ge(&Value::Rational(Ratio::new(1, 2))).unwrap(), true);
    assert_eq!(Value::Int(0).ge(&Value::Rational(Ratio::new(1, 2))).unwrap(), false);
}

#[test]
fn ge_rational_int() {
    assert_eq!(Value::Rational(Ratio::new(3, 2)).ge(&Value::Int(1)).unwrap(), true);
    assert_eq!(Value::Rational(Ratio::new(1, 2)).ge(&Value::Int(1)).unwrap(), false);
}

#[test]
fn ge_hex_int() {
    assert_eq!(Value::Hex(5).ge(&Value::Int(5)).unwrap(), true);
    assert_eq!(Value::Hex(4).ge(&Value::Int(5)).unwrap(), false);
}

#[test]
fn ge_binary_int() {
    assert_eq!(Value::Binary(5).ge(&Value::Int(5)).unwrap(), true);
    assert_eq!(Value::Binary(4).ge(&Value::Int(5)).unwrap(), false);
}

// ============================================================================
// Value::eq and Value::ne
// ============================================================================

#[test]
fn eq_int_true() {
    assert_eq!(Value::Int(5).eq(&Value::Int(5)).unwrap(), true);
}

#[test]
fn eq_int_false() {
    assert_eq!(Value::Int(5).eq(&Value::Int(3)).unwrap(), false);
}

#[test]
fn ne_int_true() {
    assert_eq!(Value::Int(5).ne(&Value::Int(3)).unwrap(), true);
}

#[test]
fn ne_int_false() {
    assert_eq!(Value::Int(5).ne(&Value::Int(5)).unwrap(), false);
}

// ============================================================================
// Value::is_truthy
// ============================================================================

#[test]
fn truthy_bool_true() {
    assert_eq!(Value::Bool(true).is_truthy(), true);
}

#[test]
fn truthy_bool_false() {
    assert_eq!(Value::Bool(false).is_truthy(), false);
}

#[test]
fn truthy_int_nonzero() {
    assert_eq!(Value::Int(1).is_truthy(), true);
    assert_eq!(Value::Int(-1).is_truthy(), true);
}

#[test]
fn truthy_int_zero() {
    assert_eq!(Value::Int(0).is_truthy(), false);
}

#[test]
fn truthy_float_nonzero() {
    assert_eq!(Value::Float(0.1).is_truthy(), true);
}

#[test]
fn truthy_float_zero() {
    assert_eq!(Value::Float(0.0).is_truthy(), false);
}

#[test]
fn truthy_rational_nonzero() {
    assert_eq!(Value::Rational(Ratio::new(1, 2)).is_truthy(), true);
}

#[test]
fn truthy_rational_zero() {
    assert_eq!(Value::Rational(Ratio::new(0, 1)).is_truthy(), false);
}

#[test]
fn truthy_string_nonempty() {
    assert_eq!(Value::String("hello".to_string()).is_truthy(), true);
}

#[test]
fn truthy_string_empty() {
    assert_eq!(Value::String(String::new()).is_truthy(), false);
}

#[test]
fn truthy_list_nonempty() {
    assert_eq!(Value::List(vec![Value::Int(1)]).is_truthy(), true);
}

#[test]
fn truthy_list_empty() {
    assert_eq!(Value::List(vec![]).is_truthy(), false);
}

#[test]
fn truthy_unit_is_false() {
    assert_eq!(Value::Unit.is_truthy(), false);
}

#[test]
fn truthy_hex_is_true() {
    assert_eq!(Value::Hex(1).is_truthy(), true);
}

#[test]
fn truthy_complex_is_true() {
    assert_eq!(Value::Complex(Complex64::new(1.0, 0.0)).is_truthy(), true);
}

// ============================================================================
// Interpreter: control flow evaluation
// ============================================================================

#[test]
fn interp_if_true_branch() {
    let code = "x = 5\nif x > 3 { y = 1 } else { y = 0 }";
    let program = parse_program(code).unwrap();
    let mut interp = Interpreter::new();
    interp.run(&program).unwrap();
    assert_eq!(interp.get_variable("y").unwrap(), Value::Int(1));
}

#[test]
fn interp_if_false_branch() {
    let code = "x = 1\nif x > 3 { y = 1 } else { y = 0 }";
    let program = parse_program(code).unwrap();
    let mut interp = Interpreter::new();
    interp.run(&program).unwrap();
    assert_eq!(interp.get_variable("y").unwrap(), Value::Int(0));
}

#[test]
fn interp_while_loop_counts() {
    let code = "i = 0\nsum = 0\nwhile i < 5 { sum = sum + i\ni = i + 1 }";
    let program = parse_program(code).unwrap();
    let mut interp = Interpreter::new();
    interp.run(&program).unwrap();
    // sum = 0 + 1 + 2 + 3 + 4 = 10
    assert_eq!(interp.get_variable("sum").unwrap(), Value::Int(10));
}

#[test]
fn interp_for_loop_range() {
    let code = "sum = 0\nfor i in 0..5 { sum = sum + i }";
    let program = parse_program(code).unwrap();
    let mut interp = Interpreter::new();
    interp.run(&program).unwrap();
    assert_eq!(interp.get_variable("sum").unwrap(), Value::Int(10));
}

#[test]
fn interp_nested_if() {
    let code = "x = 50\nif x > 100 { r = 3 } else { if x > 10 { r = 2 } else { r = 1 } }";
    let program = parse_program(code).unwrap();
    let mut interp = Interpreter::new();
    interp.run(&program).unwrap();
    assert_eq!(interp.get_variable("r").unwrap(), Value::Int(2));
}

#[test]
fn interp_print_does_not_crash() {
    let code = "print(42)";
    let program = parse_program(code).unwrap();
    let mut interp = Interpreter::new();
    // print should execute without error (output goes to stdout)
    assert!(interp.run(&program).is_ok());
}

// ============================================================================
// Formatter: verify output correctness for mutation-targeted areas
// ============================================================================

#[test]
fn formatter_simple_assignment() {
    let code = "x = 5 + 3";
    let program = parse_program(code).unwrap();
    let mut formatter = Formatter::new();
    let output = formatter.format_program(&program);
    assert!(output.contains("x = "));
    assert!(output.contains("5"));
    assert!(output.contains("+"));
    assert!(output.contains("3"));
}

#[test]
fn formatter_reverse_block() {
    let code = "reverse { x += 5 }";
    let program = parse_program(code).unwrap();
    let mut formatter = Formatter::new();
    let output = formatter.format_program(&program);
    assert!(output.contains("reverse"));
    assert!(output.contains("+="));
    assert!(output.contains("5"));
}

#[test]
fn formatter_function_with_purity() {
    let code = "@pure fn add(a: Int, b: Int): Int { return a + b }";
    let program = parse_program(code).unwrap();
    let mut formatter = Formatter::new();
    let output = formatter.format_program(&program);
    assert!(output.contains("@pure"));
    assert!(output.contains("fn add"));
}

// ============================================================================
// Pretty printer: verify output for mutation-targeted areas
// ============================================================================

#[test]
fn pretty_print_number_types() {
    let printer = PrettyPrinter::new();
    assert_eq!(printer.print_data_expr(&DataExpr::Number(Number::Int(42))), "42");
    assert_eq!(printer.print_data_expr(&DataExpr::Number(Number::Float(3.14))), "3.14");
    assert_eq!(printer.print_data_expr(&DataExpr::StringLit("hello".to_string())), "\"hello\"");
    assert_eq!(printer.print_data_expr(&DataExpr::Identifier("x".to_string())), "x");
}

#[test]
fn pretty_print_add() {
    let printer = PrettyPrinter::new();
    let expr = DataExpr::Add(
        Box::new(DataExpr::Number(Number::Int(1))),
        Box::new(DataExpr::Number(Number::Int(2))),
    );
    assert_eq!(printer.print_data_expr(&expr), "1 + 2");
}

#[test]
fn pretty_print_negate() {
    let printer = PrettyPrinter::new();
    let expr = DataExpr::Negate(Box::new(DataExpr::Number(Number::Int(5))));
    assert_eq!(printer.print_data_expr(&expr), "-5");
}

#[test]
fn pretty_print_list() {
    let printer = PrettyPrinter::new();
    let expr = DataExpr::List(vec![
        DataExpr::Number(Number::Int(1)),
        DataExpr::Number(Number::Int(2)),
    ]);
    assert_eq!(printer.print_data_expr(&expr), "[1, 2]");
}

#[test]
fn pretty_print_tuple() {
    let printer = PrettyPrinter::new();
    let expr = DataExpr::Tuple(vec![
        DataExpr::Number(Number::Int(1)),
        DataExpr::Number(Number::Int(2)),
    ]);
    assert_eq!(printer.print_data_expr(&expr), "(1, 2)");
}

#[test]
fn pretty_print_function_call() {
    let printer = PrettyPrinter::new();
    let expr = DataExpr::FunctionCall(FunctionCall {
        module: None,
        name: "add".to_string(),
        args: vec![
            DataExpr::Number(Number::Int(1)),
            DataExpr::Number(Number::Int(2)),
        ],
    });
    assert_eq!(printer.print_data_expr(&expr), "add(1, 2)");
}

#[test]
fn pretty_print_qualified_function_call() {
    let printer = PrettyPrinter::new();
    let expr = DataExpr::FunctionCall(FunctionCall {
        module: Some(vec!["Math".to_string()]),
        name: "add".to_string(),
        args: vec![DataExpr::Number(Number::Int(1))],
    });
    assert_eq!(printer.print_data_expr(&expr), "Math.add(1)");
}

// ============================================================================
// FunctionCall::qualified_name
// ============================================================================

#[test]
fn qualified_name_no_module() {
    let fc = FunctionCall { module: None, name: "foo".to_string(), args: vec![] };
    assert_eq!(fc.qualified_name(), "foo");
}

#[test]
fn qualified_name_with_module() {
    let fc = FunctionCall {
        module: Some(vec!["Math".to_string()]),
        name: "add".to_string(),
        args: vec![],
    };
    assert_eq!(fc.qualified_name(), "Math::add");
}

#[test]
fn qualified_name_deep_module() {
    let fc = FunctionCall {
        module: Some(vec!["Std".to_string(), "Math".to_string()]),
        name: "sin".to_string(),
        args: vec![],
    };
    assert_eq!(fc.qualified_name(), "Std::Math::sin");
}

// ============================================================================
// Formatter: indentation correctness (kills indent_level += / -= mutations)
// ============================================================================

#[test]
fn formatter_if_indentation() {
    let code = "if x > 0 { y = 1 }";
    let program = parse_program(code).unwrap();
    let mut formatter = Formatter::new();
    let output = formatter.format_program(&program);
    // Body should be indented (4 spaces by default)
    assert!(output.contains("    y = 1"), "If body should be indented:\n{}", output);
    // Closing brace should NOT be indented
    let lines: Vec<&str> = output.lines().collect();
    let last_non_empty = lines.iter().rev().find(|l| !l.trim().is_empty()).unwrap();
    assert!(last_non_empty.starts_with('}') || last_non_empty.trim() == "}", "Closing brace should be at top level:\n{}", output);
}

#[test]
fn formatter_if_else_indentation() {
    let code = "if x > 0 { y = 1 } else { y = 0 }";
    let program = parse_program(code).unwrap();
    let mut formatter = Formatter::new();
    let output = formatter.format_program(&program);
    assert!(output.contains("    y = 1"), "Then body should be indented:\n{}", output);
    assert!(output.contains("    y = 0"), "Else body should be indented:\n{}", output);
}

#[test]
fn formatter_while_indentation() {
    let code = "while x > 0 { x = x + 1 }";
    let program = parse_program(code).unwrap();
    let mut formatter = Formatter::new();
    let output = formatter.format_program(&program);
    assert!(output.contains("    x = x"), "While body should be indented:\n{}", output);
}

#[test]
fn formatter_for_indentation() {
    let code = "for i in 0..10 { x = x + 1 }";
    let program = parse_program(code).unwrap();
    let mut formatter = Formatter::new();
    let output = formatter.format_program(&program);
    assert!(output.contains("    x = x"), "For body should be indented:\n{}", output);
}

#[test]
fn formatter_reverse_indentation() {
    let code = "reverse { x += 5 }";
    let program = parse_program(code).unwrap();
    let mut formatter = Formatter::new();
    let output = formatter.format_program(&program);
    assert!(output.contains("    x += 5"), "Reverse body should be indented:\n{}", output);
}

#[test]
fn formatter_module_indentation() {
    let code = "module M { fn f() { x = 1 } }";
    let program = parse_program(code).unwrap();
    let mut formatter = Formatter::new();
    let output = formatter.format_program(&program);
    // Module body should be indented
    assert!(output.contains("    fn f"), "Module body should be indented:\n{}", output);
}

#[test]
fn formatter_function_body_indentation() {
    let code = "@pure fn add(a: Int, b: Int): Int { return a + b }";
    let program = parse_program(code).unwrap();
    let mut formatter = Formatter::new();
    let output = formatter.format_program(&program);
    assert!(output.contains("    return"), "Function body should be indented:\n{}", output);
}

#[test]
fn formatter_import() {
    let code = "import Math";
    let program = parse_program(code).unwrap();
    let mut formatter = Formatter::new();
    let output = formatter.format_program(&program);
    assert!(output.contains("import Math"), "Import should be formatted:\n{}", output);
}

#[test]
fn formatter_print_multiple_args() {
    let code = "print(1, 2, 3)";
    let program = parse_program(code).unwrap();
    let mut formatter = Formatter::new();
    let output = formatter.format_program(&program);
    assert!(output.contains("print("), "Print should be formatted");
    assert!(output.contains(", "), "Multiple args should have comma separator");
}

#[test]
fn formatter_type_annotation_list() {
    let code = "fn f(x: List<Int>) { return x }";
    let program = parse_program(code).unwrap();
    let mut formatter = Formatter::new();
    let output = formatter.format_program(&program);
    assert!(output.contains("List<Int>"), "List type annotation:\n{}", output);
}

// ============================================================================
// Interpreter: trace and output capture (kills enable/disable/get mutations)
// ============================================================================

#[test]
fn interp_trace_enabled() {
    let code = "x = 5";
    let program = parse_program(code).unwrap();
    let mut interp = Interpreter::new();
    interp.enable_trace();
    interp.run(&program).unwrap();
    let trace = interp.get_trace();
    assert!(!trace.is_empty(), "Trace should capture entries when enabled");
}

#[test]
fn interp_trace_disabled_by_default() {
    let code = "x = 5";
    let program = parse_program(code).unwrap();
    let mut interp = Interpreter::new();
    interp.run(&program).unwrap();
    let trace = interp.get_trace();
    assert!(trace.is_empty(), "Trace should be empty when not enabled");
}

#[test]
fn interp_output_capture() {
    let code = "print(42)";
    let program = parse_program(code).unwrap();
    let mut interp = Interpreter::new();
    interp.enable_output_capture();
    interp.run(&program).unwrap();
    let output = interp.get_output();
    assert!(!output.is_empty(), "Output capture should capture print statements");
    assert!(output[0].contains("42"), "Output should contain printed value");
}

#[test]
fn interp_output_not_captured_by_default() {
    let code = "print(42)";
    let program = parse_program(code).unwrap();
    let mut interp = Interpreter::new();
    interp.run(&program).unwrap();
    let output = interp.get_output();
    assert!(output.is_empty(), "Output should not be captured by default");
}

#[test]
fn interp_for_loop_step() {
    // For loop with explicit step
    let code = "sum = 0\nfor i in 0..10..2 { sum = sum + i }";
    let program = parse_program(code).unwrap();
    let mut interp = Interpreter::new();
    interp.run(&program).unwrap();
    // 0 + 2 + 4 + 6 + 8 = 20
    assert_eq!(interp.get_variable("sum").unwrap(), Value::Int(20));
}

// ============================================================================
// Value::Display formatting
// ============================================================================

#[test]
fn display_value_variants() {
    assert_eq!(format!("{}", Value::Int(42)), "42");
    assert_eq!(format!("{}", Value::Float(3.14)), "3.14");
    assert_eq!(format!("{}", Value::Rational(Ratio::new(1, 3))), "1/3");
    assert_eq!(format!("{}", Value::Hex(255)), "0xff");
    assert_eq!(format!("{}", Value::Binary(10)), "0b1010");
    assert_eq!(format!("{}", Value::Symbolic("x".to_string())), "x");
    assert_eq!(format!("{}", Value::Bool(true)), "true");
    assert_eq!(format!("{}", Value::String("hi".to_string())), "\"hi\"");
    assert_eq!(format!("{}", Value::Unit), "()");
}

#[test]
fn display_complex_positive_imag() {
    let c = Value::Complex(Complex64::new(1.0, 2.0));
    let s = format!("{}", c);
    assert!(s.contains("+"), "Positive imag should use +: {}", s);
}

#[test]
fn display_complex_negative_imag() {
    let c = Value::Complex(Complex64::new(1.0, -2.0));
    let s = format!("{}", c);
    assert!(s.contains("-"), "Negative imag should use -: {}", s);
}

#[test]
fn display_list() {
    let l = Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(3)]);
    assert_eq!(format!("{}", l), "[1, 2, 3]");
}

#[test]
fn display_tuple() {
    let t = Value::Tuple(vec![Value::Int(1), Value::Int(2)]);
    assert_eq!(format!("{}", t), "(1, 2)");
}
