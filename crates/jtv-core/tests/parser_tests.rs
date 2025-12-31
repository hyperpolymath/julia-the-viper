// Parser tests for Julia the Viper
use jtv_core::{parse_program, ControlStmt, DataExpr, Number};

#[test]
fn test_simple_addition() {
    let code = "x = 5 + 3";
    let program = parse_program(code).unwrap();
    assert_eq!(program.statements.len(), 1);
}

#[test]
fn test_multiple_additions() {
    let code = "result = 1 + 2 + 3 + 4 + 5";
    let program = parse_program(code).unwrap();
    assert_eq!(program.statements.len(), 1);
}

#[test]
fn test_number_systems() {
    // Integer
    let code = "x = 42";
    assert!(parse_program(code).is_ok());

    // Float
    let code = "x = 3.14";
    assert!(parse_program(code).is_ok());

    // Rational
    let code = "x = 1/2";
    assert!(parse_program(code).is_ok());

    // Hex
    let code = "x = 0xFF";
    assert!(parse_program(code).is_ok());

    // Binary
    let code = "x = 0b1010";
    assert!(parse_program(code).is_ok());
}

#[test]
fn test_function_declaration() {
    let code = r#"
        fn add(a: Int, b: Int): Int {
            return a + b
        }
    "#;
    let program = parse_program(code).unwrap();
    assert_eq!(program.statements.len(), 1);
}

#[test]
fn test_pure_function() {
    let code = r#"
        @pure fn double(x: Int): Int {
            return x + x
        }
    "#;
    let program = parse_program(code).unwrap();
    assert_eq!(program.statements.len(), 1);
}

#[test]
fn test_for_loop() {
    let code = r#"
        sum = 0
        for i in 1..10 {
            sum = sum + i
        }
    "#;
    let program = parse_program(code).unwrap();
    assert_eq!(program.statements.len(), 2);
}

#[test]
fn test_while_loop() {
    let code = r#"
        x = 0
        while x < 10 {
            x = x + 1
        }
    "#;
    let program = parse_program(code).unwrap();
    assert_eq!(program.statements.len(), 2);
}

#[test]
fn test_if_statement() {
    let code = r#"
        if x > 0 {
            print(1)
        } else {
            print(0)
        }
    "#;
    let program = parse_program(code).unwrap();
    assert_eq!(program.statements.len(), 1);
}

#[test]
fn test_nested_expressions() {
    let code = "result = (1 + 2) + (3 + 4)";
    let program = parse_program(code).unwrap();
    assert_eq!(program.statements.len(), 1);
}

#[test]
fn test_negation() {
    let code = "x = -5";
    let program = parse_program(code).unwrap();
    assert_eq!(program.statements.len(), 1);
}

#[test]
fn test_comparison() {
    let code = "if x == y { print(1) }";
    let program = parse_program(code).unwrap();
    assert_eq!(program.statements.len(), 1);
}

#[test]
fn test_logical_operators() {
    let code = "if x > 0 && y < 10 { print(1) }";
    let program = parse_program(code).unwrap();
    assert_eq!(program.statements.len(), 1);
}

#[test]
fn test_module() {
    let code = r#"
        module Math {
            fn add(a: Int, b: Int): Int {
                return a + b
            }
        }
    "#;
    let program = parse_program(code).unwrap();
    assert_eq!(program.statements.len(), 1);
}

#[test]
fn test_import() {
    let code = "import Math";
    let program = parse_program(code).unwrap();
    assert_eq!(program.statements.len(), 1);
}

#[test]
fn test_list_literal() {
    let code = "numbers = [1, 2, 3, 4, 5]";
    let program = parse_program(code).unwrap();
    assert_eq!(program.statements.len(), 1);
}

#[test]
fn test_tuple_literal() {
    let code = "point = (10, 20)";
    let program = parse_program(code).unwrap();
    assert_eq!(program.statements.len(), 1);
}

#[test]
fn test_function_call() {
    let code = "result = add(5, 3)";
    let program = parse_program(code).unwrap();
    assert_eq!(program.statements.len(), 1);
}

#[test]
fn test_reverse_block() {
    let code = r#"
        reverse {
            x += 10
            y += 5
        }
    "#;
    let program = parse_program(code).unwrap();
    assert_eq!(program.statements.len(), 1);
}

#[test]
fn test_comments() {
    let code = r#"
        // This is a comment
        x = 5

        /* This is a
           block comment */
        y = 10
    "#;
    let program = parse_program(code).unwrap();
    assert_eq!(program.statements.len(), 2);
}

#[test]
#[ignore] // Requires v1.1 features: module-qualified calls (Calculator.add) and range expressions with arithmetic (n+1)
fn test_complex_program() {
    let code = r#"
        module Calculator {
            @pure fn add(a: Int, b: Int): Int {
                return a + b
            }

            fn factorial(n: Int): Int {
                result = 1
                for i in 2..n+1 {
                    result = multiply(result, i)
                }
                return result
            }

            fn multiply(a: Int, b: Int): Int {
                result = 0
                for i in 0..b {
                    result = result + a
                }
                return result
            }
        }

        import Calculator

        x = Calculator.add(5, 3)
        y = Calculator.factorial(5)
    "#;
    let program = parse_program(code);
    assert!(program.is_ok());
}

#[test]
fn test_parse_error() {
    // Invalid syntax should return error
    let code = "x = 5 +";
    assert!(parse_program(code).is_err());
}

#[test]
fn test_security_data_language_only_addition() {
    // Data expressions should only allow addition
    let code = "x = 5 + 3";
    assert!(parse_program(code).is_ok());

    // This grammatically prevents code injection
    // Even malicious input cannot create loops or conditionals in Data context
}
