// SPDX-License-Identifier: MIT OR GPL-3.0-or-later OR Palimpsest-0.8
// SPDX-FileCopyrightText: 2025 Julia the Viper Contributors
//
// Julia the Viper - Standard Library
//
// This module provides the StdLib registry that integrates:
// - Common library functions (math, collections) - language-agnostic
// - JtV-specific functions (number systems, reversible) - JtV unique features

use crate::number::Value;
use crate::error::{JtvError, Result};
use crate::libraries::common::{math, collections};
use crate::libraries::jtv::{number_systems, reversible};
use std::collections::HashMap;

/// Built-in function signature
pub type BuiltinFn = fn(&[Value]) -> Result<Value>;

/// Standard library registry
pub struct StdLib {
    functions: HashMap<String, (BuiltinFn, usize)>,  // (function, arity)
}

impl StdLib {
    pub fn new() -> Self {
        let mut lib = StdLib {
            functions: HashMap::new(),
        };
        // Common library (language-agnostic)
        lib.register_common_math();
        lib.register_common_collections();
        // JtV-specific library
        lib.register_jtv_number_systems();
        lib.register_jtv_reversible();
        lib
    }

    pub fn get(&self, name: &str) -> Option<&(BuiltinFn, usize)> {
        self.functions.get(name)
    }

    pub fn call(&self, name: &str, args: &[Value]) -> Result<Value> {
        if let Some((func, arity)) = self.functions.get(name) {
            if args.len() != *arity {
                return Err(JtvError::ArityMismatch {
                    expected: *arity,
                    got: args.len(),
                });
            }
            func(args)
        } else {
            Err(JtvError::UndefinedFunction(name.to_string()))
        }
    }

    pub fn has(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }

    pub fn list_functions(&self) -> Vec<&String> {
        self.functions.keys().collect()
    }

    fn register(&mut self, name: &str, func: BuiltinFn, arity: usize) {
        self.functions.insert(name.to_string(), (func, arity));
    }

    // ===== Common Library: Math =====
    fn register_common_math(&mut self) {
        self.register("abs", math::abs, 1);
        self.register("max", math::max, 2);
        self.register("min", math::min, 2);
        self.register("sign", math::sign, 1);
        self.register("clamp", math::clamp, 3);
        self.register("floor", math::floor, 1);
        self.register("ceil", math::ceil, 1);
        self.register("round", math::round, 1);
        self.register("gcd", math::gcd, 2);
        self.register("lcm", math::lcm, 2);
        self.register("factorial", math::factorial, 1);
        self.register("isPrime", math::is_prime, 1);
        self.register("pow", math::pow, 2);
        self.register("sqrt", math::sqrt, 1);
        self.register("mod", math::modulo, 2);
    }

    // ===== Common Library: Collections =====
    fn register_common_collections(&mut self) {
        self.register("length", collections::length, 1);
        self.register("sum", collections::sum, 1);
        self.register("product", collections::product, 1);
        self.register("head", collections::head, 1);
        self.register("tail", collections::tail, 1);
        self.register("last", collections::last, 1);
        self.register("init", collections::init, 1);
        self.register("reverse", collections::reverse, 1);
        self.register("range", collections::range, 2);
        self.register("concat", collections::concat, 2);
        self.register("contains", collections::contains, 2);
        self.register("at", collections::at, 2);
        self.register("take", collections::take, 2);
        self.register("drop", collections::drop, 2);
        self.register("zip", collections::zip, 2);
        self.register("findMin", collections::find_min, 1);
        self.register("findMax", collections::find_max, 1);
    }

    // ===== JtV Library: Number Systems =====
    fn register_jtv_number_systems(&mut self) {
        self.register("rational", number_systems::rational, 2);
        self.register("numerator", number_systems::numerator, 1);
        self.register("denominator", number_systems::denominator, 1);
        self.register("reduce", number_systems::reduce, 1);
        self.register("complex", number_systems::complex, 2);
        self.register("realPart", number_systems::real_part, 1);
        self.register("imagPart", number_systems::imag_part, 1);
        self.register("magnitude", number_systems::magnitude, 1);
        self.register("phase", number_systems::phase, 1);
        self.register("conjugate", number_systems::conjugate, 1);
        self.register("toHex", number_systems::to_hex, 1);
        self.register("toBinary", number_systems::to_binary, 1);
        self.register("toInt", number_systems::to_int, 1);
        self.register("toFloat", number_systems::to_float, 1);
        self.register("symbolic", number_systems::symbolic, 1);
        self.register("isSymbolic", number_systems::is_symbolic, 1);
        self.register("isRational", number_systems::is_rational, 1);
        self.register("isComplex", number_systems::is_complex, 1);
        self.register("isInt", number_systems::is_int, 1);
        self.register("isFloat", number_systems::is_float, 1);
        self.register("typeOf", number_systems::type_of, 1);
    }

    // ===== JtV Library: Reversible Computing =====
    fn register_jtv_reversible(&mut self) {
        self.register("subtract", reversible::subtract, 2);
        self.register("increment", reversible::increment, 1);
        self.register("decrement", reversible::decrement, 1);
        self.register("identity", reversible::identity, 1);
        self.register("swap", reversible::swap, 2);
        self.register("cnot", reversible::cnot, 2);
        self.register("isInvertible", reversible::is_invertible, 1);
        self.register("additiveInverse", reversible::additive_inverse, 1);
        self.register("xor", reversible::xor, 2);
        self.register("bitCount", reversible::bit_count, 1);
    }
}

impl Default for StdLib {
    fn default() -> Self {
        Self::new()
    }
}

// Old implementations removed - now using lib/common and lib/jtv modules

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abs() {
        let lib = StdLib::new();
        assert_eq!(lib.call("abs", &[Value::Int(-5)]).unwrap(), Value::Int(5));
        assert_eq!(lib.call("abs", &[Value::Int(5)]).unwrap(), Value::Int(5));
    }

    #[test]
    fn test_max_min() {
        let lib = StdLib::new();
        assert_eq!(lib.call("max", &[Value::Int(3), Value::Int(5)]).unwrap(), Value::Int(5));
        assert_eq!(lib.call("min", &[Value::Int(3), Value::Int(5)]).unwrap(), Value::Int(3));
    }

    #[test]
    fn test_gcd() {
        let lib = StdLib::new();
        assert_eq!(lib.call("gcd", &[Value::Int(12), Value::Int(8)]).unwrap(), Value::Int(4));
    }

    #[test]
    fn test_factorial() {
        let lib = StdLib::new();
        assert_eq!(lib.call("factorial", &[Value::Int(5)]).unwrap(), Value::Int(120));
    }

    #[test]
    fn test_is_prime() {
        let lib = StdLib::new();
        assert_eq!(lib.call("isPrime", &[Value::Int(7)]).unwrap(), Value::Bool(true));
        assert_eq!(lib.call("isPrime", &[Value::Int(8)]).unwrap(), Value::Bool(false));
    }

    #[test]
    fn test_length() {
        let lib = StdLib::new();
        let list = Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(3)]);
        assert_eq!(lib.call("length", &[list]).unwrap(), Value::Int(3));
    }

    #[test]
    fn test_sum() {
        let lib = StdLib::new();
        let list = Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(3)]);
        assert_eq!(lib.call("sum", &[list]).unwrap(), Value::Int(6));
    }

    #[test]
    fn test_head_tail() {
        let lib = StdLib::new();
        let list = Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(3)]);
        assert_eq!(lib.call("head", &[list.clone()]).unwrap(), Value::Int(1));
        assert_eq!(
            lib.call("tail", &[list]).unwrap(),
            Value::List(vec![Value::Int(2), Value::Int(3)])
        );
    }

    #[test]
    fn test_range() {
        let lib = StdLib::new();
        assert_eq!(
            lib.call("range", &[Value::Int(1), Value::Int(4)]).unwrap(),
            Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(3)])
        );
    }
}
