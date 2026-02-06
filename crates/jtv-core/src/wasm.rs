// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell <jonathan.jewell@open.ac.uk>
//
// WASM bindings for Julia the Viper
//
// Exposes the full JtV language runtime to WebAssembly consumers:
//   - Parsing and AST inspection
//   - Execution with output capture
//   - Type checking
//   - Purity analysis
//   - Code formatting
//   - Reversible computing
//   - State management (variables, reset)
//   - Structured error reporting
//   - Execution tracing

// Imports used by the non-WASM analysis API (always compiled)
use crate::parser::parse_program;
use crate::purity::PurityChecker;
use crate::typechecker::TypeChecker;

// Additional imports for WASM-only bindings
#[cfg(target_arch = "wasm32")]
use crate::formatter::format_code;
#[cfg(target_arch = "wasm32")]
use crate::Interpreter;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// ---------------------------------------------------------------------------
// Stateful WASM interface: JtvWasm
// ---------------------------------------------------------------------------
// Maintains interpreter state across multiple calls, enabling REPL-like
// interaction from JavaScript. Output from print statements is captured
// into a buffer and returned as JSON arrays.

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct JtvWasm {
    interpreter: Interpreter,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl JtvWasm {
    /// Create a new JtV WASM runtime instance.
    ///
    /// Output capture is enabled by default so print statements are
    /// buffered and retrievable via `get_output()` / `run_and_collect()`.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut interpreter = Interpreter::new();
        interpreter.enable_output_capture();
        JtvWasm { interpreter }
    }

    // =======================================================================
    // Execution
    // =======================================================================

    /// Parse and execute JtV source code.
    ///
    /// Returns `"ok"` on success. Captured print output is available via
    /// `get_output()`. Throws a JS error string on parse or runtime failure.
    #[wasm_bindgen]
    pub fn run(&mut self, code: &str) -> Result<String, JsValue> {
        let program =
            parse_program(code).map_err(|e| JsValue::from_str(&format!("{}", e)))?;

        self.interpreter
            .run(&program)
            .map_err(|e| JsValue::from_str(&format!("{}", e)))?;

        Ok("ok".to_string())
    }

    /// Parse, execute, and return captured output as a JSON array of strings.
    ///
    /// This is the primary convenience method for web UIs: it runs the code
    /// and hands back everything the program printed, in order.
    ///
    /// Returns a JSON string like `["line 1","line 2"]`. The output buffer is
    /// drained after this call, so successive calls return only new output.
    #[wasm_bindgen]
    pub fn run_and_collect(&mut self, code: &str) -> Result<String, JsValue> {
        let program =
            parse_program(code).map_err(|e| JsValue::from_str(&format!("{}", e)))?;

        self.interpreter
            .run(&program)
            .map_err(|e| JsValue::from_str(&format!("{}", e)))?;

        let output = self.interpreter.take_output();
        serde_json::to_string(&output)
            .map_err(|e| JsValue::from_str(&format!("{}", e)))
    }

    // =======================================================================
    // Output capture
    // =======================================================================

    /// Retrieve captured output lines as a JSON array of strings, then clear
    /// the buffer.
    #[wasm_bindgen]
    pub fn get_output(&mut self) -> Result<String, JsValue> {
        let output = self.interpreter.take_output();
        serde_json::to_string(&output)
            .map_err(|e| JsValue::from_str(&format!("{}", e)))
    }

    // =======================================================================
    // Parsing & AST
    // =======================================================================

    /// Parse source code and return the AST as pretty-printed JSON.
    ///
    /// Does **not** execute the code or modify interpreter state.
    #[wasm_bindgen]
    pub fn parse_only(&self, code: &str) -> Result<String, JsValue> {
        let program =
            parse_program(code).map_err(|e| JsValue::from_str(&format!("{}", e)))?;

        serde_json::to_string_pretty(&program)
            .map_err(|e| JsValue::from_str(&format!("{}", e)))
    }

    /// Validate source code (parse only). Returns `true` if the code parses
    /// successfully, `false` otherwise.
    #[wasm_bindgen]
    pub fn validate(&self, code: &str) -> bool {
        parse_program(code).is_ok()
    }

    // =======================================================================
    // Type checking
    // =======================================================================

    /// Run the static type checker on source code.
    ///
    /// Returns `"ok"` if the code is well-typed. Throws a JS error string
    /// describing the first type error found otherwise.
    #[wasm_bindgen]
    pub fn type_check(&self, code: &str) -> Result<String, JsValue> {
        let program =
            parse_program(code).map_err(|e| JsValue::from_str(&format!("{}", e)))?;

        let mut checker = TypeChecker::new();
        checker
            .check_program(&program)
            .map_err(|e| JsValue::from_str(&format!("{}", e)))?;

        Ok("ok".to_string())
    }

    // =======================================================================
    // Purity analysis
    // =======================================================================

    /// Run the purity checker on source code.
    ///
    /// Verifies that `@total` and `@pure` annotations are respected.
    /// Returns `"ok"` if all purity contracts hold. Throws a JS error string
    /// describing the first purity violation found otherwise.
    #[wasm_bindgen]
    pub fn purity_check(&self, code: &str) -> Result<String, JsValue> {
        let program =
            parse_program(code).map_err(|e| JsValue::from_str(&format!("{}", e)))?;

        let mut checker = PurityChecker::new();
        checker
            .check_program(&program)
            .map_err(|e| JsValue::from_str(&format!("{}", e)))?;

        Ok("ok".to_string())
    }

    // =======================================================================
    // Code formatting
    // =======================================================================

    /// Format source code using the default JtV formatter.
    ///
    /// Returns the pretty-printed source string. Throws a JS error on
    /// parse failure.
    #[wasm_bindgen]
    pub fn format(&self, code: &str) -> Result<String, JsValue> {
        format_code(code).map_err(|e| JsValue::from_str(&e))
    }

    // =======================================================================
    // Full analysis pipeline
    // =======================================================================

    /// Run all static analyses (parse, type check, purity check) and return
    /// a structured JSON report.
    ///
    /// The returned JSON object has the shape:
    /// ```json
    /// {
    ///   "parse": "ok" | "<error>",
    ///   "type_check": "ok" | "<error>" | null,
    ///   "purity_check": "ok" | "<error>" | null
    /// }
    /// ```
    ///
    /// Downstream checks are `null` if an earlier phase failed (a parse
    /// error prevents type/purity checking).
    #[wasm_bindgen]
    pub fn analyze(&self, code: &str) -> Result<String, JsValue> {
        let parse_result = parse_program(code);

        let (parse_status, program) = match parse_result {
            Ok(prog) => ("ok".to_string(), Some(prog)),
            Err(e) => (format!("{}", e), None),
        };

        let type_status = program.as_ref().map(|prog| {
            let mut checker = TypeChecker::new();
            match checker.check_program(prog) {
                Ok(()) => "ok".to_string(),
                Err(e) => format!("{}", e),
            }
        });

        let purity_status = program.as_ref().map(|prog| {
            let mut checker = PurityChecker::new();
            match checker.check_program(prog) {
                Ok(()) => "ok".to_string(),
                Err(e) => format!("{}", e),
            }
        });

        let report = serde_json::json!({
            "parse": parse_status,
            "type_check": type_status,
            "purity_check": purity_status,
        });

        serde_json::to_string_pretty(&report)
            .map_err(|e| JsValue::from_str(&format!("{}", e)))
    }

    // =======================================================================
    // Variable inspection
    // =======================================================================

    /// Get the string representation of a single variable.
    ///
    /// Throws a JS error if the variable is not defined.
    #[wasm_bindgen]
    pub fn get_variable(&self, name: &str) -> Result<String, JsValue> {
        self.interpreter
            .get_variable(name)
            .map(|v| format!("{}", v))
            .map_err(|e| JsValue::from_str(&format!("{}", e)))
    }

    /// Return all defined variables as a JSON object `{ "name": "value_str", ... }`.
    #[wasm_bindgen]
    pub fn get_all_variables(&self) -> Result<String, JsValue> {
        let vars = self.interpreter.get_variables();
        let map: std::collections::HashMap<String, String> = vars
            .into_iter()
            .map(|(k, v)| (k, format!("{}", v)))
            .collect();
        serde_json::to_string_pretty(&map)
            .map_err(|e| JsValue::from_str(&format!("{}", e)))
    }

    /// Get the last evaluated result as a string, or `null` if there is none.
    #[wasm_bindgen]
    pub fn get_last_result(&self) -> Result<String, JsValue> {
        match self.interpreter.get_last_result() {
            Some(v) => Ok(format!("{}", v)),
            None => Ok("null".to_string()),
        }
    }

    // =======================================================================
    // Execution tracing
    // =======================================================================

    /// Enable per-statement execution tracing.
    #[wasm_bindgen]
    pub fn enable_trace(&mut self) {
        self.interpreter.enable_trace();
    }

    /// Disable execution tracing and clear the trace buffer.
    #[wasm_bindgen]
    pub fn disable_trace(&mut self) {
        self.interpreter.disable_trace();
    }

    /// Get the execution trace as a JSON array of trace entries.
    ///
    /// Each entry has `{ "stmt_type": "...", "line": "...", "env": { ... } }`.
    #[wasm_bindgen]
    pub fn get_trace(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(self.interpreter.get_trace())
            .map_err(|e| JsValue::from_str(&format!("{}", e)))
    }

    // =======================================================================
    // State management
    // =======================================================================

    /// Reset the interpreter to a clean slate (clears all variables,
    /// functions, modules, trace, and output buffer).
    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.interpreter.reset();
    }
}

// ---------------------------------------------------------------------------
// Standalone (stateless) convenience functions
// ---------------------------------------------------------------------------
// These do not require constructing a `JtvWasm` instance. Handy for
// one-shot operations from JavaScript.

/// Return the jtv-core crate version string.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn jtv_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Parse JtV source code and return the AST as JSON.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn jtv_parse(code: &str) -> Result<String, JsValue> {
    let program =
        parse_program(code).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    serde_json::to_string_pretty(&program)
        .map_err(|e| JsValue::from_str(&format!("{}", e)))
}

/// Format JtV source code and return the formatted string.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn jtv_format(code: &str) -> Result<String, JsValue> {
    format_code(code).map_err(|e| JsValue::from_str(&e))
}

/// Type-check JtV source code. Returns `"ok"` or throws an error string.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn jtv_type_check(code: &str) -> Result<String, JsValue> {
    let program =
        parse_program(code).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    let mut checker = TypeChecker::new();
    checker
        .check_program(&program)
        .map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    Ok("ok".to_string())
}

/// Purity-check JtV source code. Returns `"ok"` or throws an error string.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn jtv_purity_check(code: &str) -> Result<String, JsValue> {
    let program =
        parse_program(code).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    let mut checker = PurityChecker::new();
    checker
        .check_program(&program)
        .map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    Ok("ok".to_string())
}

/// Run full analysis pipeline (parse + type check + purity check) and
/// return a structured JSON report.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn jtv_analyze(code: &str) -> Result<String, JsValue> {
    let wasm = JtvWasm::new();
    wasm.analyze(code)
}

/// Execute JtV source code and return captured output as a JSON array of
/// strings. This is the simplest one-shot execution entry point.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn jtv_run(code: &str) -> Result<String, JsValue> {
    let mut wasm = JtvWasm::new();
    wasm.run_and_collect(code)
}

// ---------------------------------------------------------------------------
// Non-WASM API surface (available on all targets for testing)
// ---------------------------------------------------------------------------
// These types and functions are always compiled so that native tests can
// exercise the WASM API logic without needing a wasm32 target.

/// Structured analysis result returned by `analyze_code`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AnalysisReport {
    /// `"ok"` or the parse error message.
    pub parse: String,
    /// `"ok"`, a type error message, or `None` if parsing failed.
    pub type_check: Option<String>,
    /// `"ok"`, a purity error message, or `None` if parsing failed.
    pub purity_check: Option<String>,
}

/// Run the full analysis pipeline on native targets (non-WASM).
///
/// Returns an `AnalysisReport` with results for each phase.
pub fn analyze_code(code: &str) -> AnalysisReport {
    let parse_result = parse_program(code);

    let (parse_status, program) = match parse_result {
        Ok(prog) => ("ok".to_string(), Some(prog)),
        Err(e) => (format!("{}", e), None),
    };

    let type_status = program.as_ref().map(|prog| {
        let mut checker = TypeChecker::new();
        match checker.check_program(prog) {
            Ok(()) => "ok".to_string(),
            Err(e) => format!("{}", e),
        }
    });

    let purity_status = program.as_ref().map(|prog| {
        let mut checker = PurityChecker::new();
        match checker.check_program(prog) {
            Ok(()) => "ok".to_string(),
            Err(e) => format!("{}", e),
        }
    });

    AnalysisReport {
        parse: parse_status,
        type_check: type_status,
        purity_check: purity_status,
    }
}

// ---------------------------------------------------------------------------
// Tests (run on native target)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formatter::format_code;
    use crate::Interpreter;

    #[test]
    fn test_output_capture() {
        let code = r#"
            print(42)
            print(1 + 2)
        "#;
        let program = crate::parse_program(code).unwrap();
        let mut interp = Interpreter::new();
        interp.enable_output_capture();
        interp.run(&program).unwrap();

        let output = interp.take_output();
        assert_eq!(output.len(), 2);
        assert_eq!(output[0], "42");
        assert_eq!(output[1], "3");
    }

    #[test]
    fn test_interpreter_reset() {
        let code = "x = 10";
        let program = crate::parse_program(code).unwrap();
        let mut interp = Interpreter::new();
        interp.run(&program).unwrap();
        assert!(interp.get_variable("x").is_ok());

        interp.reset();
        assert!(interp.get_variable("x").is_err());
    }

    #[test]
    fn test_analyze_code_valid() {
        let code = r#"
            @total fn add(a: Int, b: Int): Int {
                return a + b
            }
        "#;
        let report = analyze_code(code);
        assert_eq!(report.parse, "ok");
        assert_eq!(report.type_check, Some("ok".to_string()));
        assert_eq!(report.purity_check, Some("ok".to_string()));
    }

    #[test]
    fn test_analyze_code_parse_error() {
        let code = "fn {{{ invalid";
        let report = analyze_code(code);
        assert_ne!(report.parse, "ok");
        // Downstream checks should be None when parse fails
        assert!(report.type_check.is_none());
        assert!(report.purity_check.is_none());
    }

    #[test]
    fn test_analyze_code_purity_violation() {
        let code = r#"
            @total fn bad(x: Int): Int {
                print(x)
                return x
            }
        "#;
        let report = analyze_code(code);
        assert_eq!(report.parse, "ok");
        assert_eq!(report.type_check, Some("ok".to_string()));
        // Purity check should fail: @total with print is a violation
        assert!(report.purity_check.as_ref().unwrap() != "ok");
    }

    #[test]
    fn test_format_via_wasm_api() {
        let code = "x=5+3";
        let formatted = format_code(code).unwrap();
        assert_eq!(formatted, "x = 5 + 3\n");
    }

    #[test]
    fn test_output_capture_for_loop() {
        let code = r#"
            for i in 0..3 {
                print(i)
            }
        "#;
        let program = crate::parse_program(code).unwrap();
        let mut interp = Interpreter::new();
        interp.enable_output_capture();
        interp.run(&program).unwrap();

        let output = interp.take_output();
        assert_eq!(output, vec!["0", "1", "2"]);
    }

    #[test]
    fn test_get_all_variables() {
        let code = r#"
            x = 10
            y = 20
            z = x + y
        "#;
        let program = crate::parse_program(code).unwrap();
        let mut interp = Interpreter::new();
        interp.run(&program).unwrap();

        let vars = interp.get_variables();
        let var_names: Vec<&str> = vars.iter().map(|(k, _)| k.as_str()).collect();
        assert!(var_names.contains(&"x"));
        assert!(var_names.contains(&"y"));
        assert!(var_names.contains(&"z"));
    }

    #[test]
    fn test_last_result() {
        let code = "result = 42";
        let program = crate::parse_program(code).unwrap();
        let mut interp = Interpreter::new();
        interp.run(&program).unwrap();

        let last = interp.get_last_result();
        assert!(last.is_some());
        assert_eq!(format!("{}", last.unwrap()), "42");
    }

    #[test]
    fn test_output_buffer_drain_on_take() {
        let code1 = "print(1)";
        let code2 = "print(2)";
        let program1 = crate::parse_program(code1).unwrap();
        let program2 = crate::parse_program(code2).unwrap();

        let mut interp = Interpreter::new();
        interp.enable_output_capture();

        interp.run(&program1).unwrap();
        let out1 = interp.take_output();
        assert_eq!(out1, vec!["1"]);

        interp.run(&program2).unwrap();
        let out2 = interp.take_output();
        assert_eq!(out2, vec!["2"]);
        // Buffer drained: second take should not include "1"
    }
}
