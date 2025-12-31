// WASM bindings for Julia the Viper
#[cfg(target_arch = "wasm32")]
use crate::{parse_program, Interpreter};
#[cfg(target_arch = "wasm32")]
use serde_json;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct JtvWasm {
    interpreter: Interpreter,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl JtvWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        JtvWasm {
            interpreter: Interpreter::new(),
        }
    }

    #[wasm_bindgen]
    pub fn run(&mut self, code: &str) -> Result<String, JsValue> {
        let program = parse_program(code).map_err(|e| JsValue::from_str(&format!("{}", e)))?;

        self.interpreter
            .run(&program)
            .map_err(|e| JsValue::from_str(&format!("{}", e)))?;

        Ok("Execution successful".to_string())
    }

    #[wasm_bindgen]
    pub fn get_variable(&self, name: &str) -> Result<String, JsValue> {
        self.interpreter
            .get_variable(name)
            .map(|v| format!("{}", v))
            .map_err(|e| JsValue::from_str(&format!("{}", e)))
    }

    #[wasm_bindgen]
    pub fn parse_only(&self, code: &str) -> Result<String, JsValue> {
        let program = parse_program(code).map_err(|e| JsValue::from_str(&format!("{}", e)))?;

        serde_json::to_string_pretty(&program).map_err(|e| JsValue::from_str(&format!("{}", e)))
    }

    #[wasm_bindgen]
    pub fn enable_trace(&mut self) {
        self.interpreter.enable_trace();
    }

    #[wasm_bindgen]
    pub fn get_trace(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(self.interpreter.get_trace())
            .map_err(|e| JsValue::from_str(&format!("{}", e)))
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn jtv_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
