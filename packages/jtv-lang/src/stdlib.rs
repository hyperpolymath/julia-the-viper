// SPDX-License-Identifier: MIT OR GPL-3.0-or-later OR Palimpsest-0.8
// SPDX-FileCopyrightText: 2025 Julia the Viper Contributors
//
// Julia the Viper - Standard Library

use crate::number::Value;
use crate::error::{JtvError, Result};
use std::collections::HashMap;
use num_traits::Signed;

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
        lib.register_prelude();
        lib.register_math();
        lib.register_collections();
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

    // ===== std.prelude (auto-imported) =====
    fn register_prelude(&mut self) {
        self.register("abs", stdlib_abs, 1);
        self.register("max", stdlib_max, 2);
        self.register("min", stdlib_min, 2);
        self.register("sign", stdlib_sign, 1);
        self.register("clamp", stdlib_clamp, 3);
        self.register("floor", stdlib_floor, 1);
        self.register("ceil", stdlib_ceil, 1);
        self.register("round", stdlib_round, 1);
    }

    // ===== std.math =====
    fn register_math(&mut self) {
        self.register("gcd", stdlib_gcd, 2);
        self.register("lcm", stdlib_lcm, 2);
        self.register("factorial", stdlib_factorial, 1);
        self.register("isPrime", stdlib_is_prime, 1);
        self.register("pow", stdlib_pow, 2);
        self.register("sqrt", stdlib_sqrt, 1);
        self.register("mod", stdlib_mod, 2);
    }

    // ===== std.collections =====
    fn register_collections(&mut self) {
        self.register("length", stdlib_length, 1);
        self.register("sum", stdlib_sum, 1);
        self.register("product", stdlib_product, 1);
        self.register("head", stdlib_head, 1);
        self.register("tail", stdlib_tail, 1);
        self.register("last", stdlib_last, 1);
        self.register("init", stdlib_init, 1);
        self.register("reverse", stdlib_reverse, 1);
        self.register("range", stdlib_range, 2);
        self.register("concat", stdlib_concat, 2);
        self.register("contains", stdlib_contains, 2);
        self.register("at", stdlib_at, 2);
    }
}

impl Default for StdLib {
    fn default() -> Self {
        Self::new()
    }
}

// ===== Prelude Functions =====

fn stdlib_abs(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::Int(n) => Ok(Value::Int(n.abs())),
        Value::Float(f) => Ok(Value::Float(f.abs())),
        Value::Rational(r) => Ok(Value::Rational(r.abs())),
        _ => Err(JtvError::TypeError("abs requires a numeric argument".to_string())),
    }
}

fn stdlib_max(args: &[Value]) -> Result<Value> {
    match (&args[0], &args[1]) {
        (Value::Int(a), Value::Int(b)) => Ok(Value::Int(*a.max(b))),
        (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.max(*b))),
        (Value::Int(a), Value::Float(b)) => Ok(Value::Float((*a as f64).max(*b))),
        (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a.max(*b as f64))),
        _ => Err(JtvError::TypeError("max requires numeric arguments".to_string())),
    }
}

fn stdlib_min(args: &[Value]) -> Result<Value> {
    match (&args[0], &args[1]) {
        (Value::Int(a), Value::Int(b)) => Ok(Value::Int(*a.min(b))),
        (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.min(*b))),
        (Value::Int(a), Value::Float(b)) => Ok(Value::Float((*a as f64).min(*b))),
        (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a.min(*b as f64))),
        _ => Err(JtvError::TypeError("min requires numeric arguments".to_string())),
    }
}

fn stdlib_sign(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::Int(n) => Ok(Value::Int(n.signum())),
        Value::Float(f) => {
            if *f > 0.0 { Ok(Value::Int(1)) }
            else if *f < 0.0 { Ok(Value::Int(-1)) }
            else { Ok(Value::Int(0)) }
        }
        _ => Err(JtvError::TypeError("sign requires a numeric argument".to_string())),
    }
}

fn stdlib_clamp(args: &[Value]) -> Result<Value> {
    match (&args[0], &args[1], &args[2]) {
        (Value::Int(x), Value::Int(lo), Value::Int(hi)) => {
            Ok(Value::Int((*x).max(*lo).min(*hi)))
        }
        (Value::Float(x), Value::Float(lo), Value::Float(hi)) => {
            Ok(Value::Float(x.max(*lo).min(*hi)))
        }
        _ => Err(JtvError::TypeError("clamp requires numeric arguments".to_string())),
    }
}

fn stdlib_floor(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::Int(n) => Ok(Value::Int(*n)),
        Value::Float(f) => Ok(Value::Int(f.floor() as i64)),
        _ => Err(JtvError::TypeError("floor requires a numeric argument".to_string())),
    }
}

fn stdlib_ceil(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::Int(n) => Ok(Value::Int(*n)),
        Value::Float(f) => Ok(Value::Int(f.ceil() as i64)),
        _ => Err(JtvError::TypeError("ceil requires a numeric argument".to_string())),
    }
}

fn stdlib_round(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::Int(n) => Ok(Value::Int(*n)),
        Value::Float(f) => Ok(Value::Int(f.round() as i64)),
        _ => Err(JtvError::TypeError("round requires a numeric argument".to_string())),
    }
}

// ===== Math Functions =====

fn stdlib_gcd(args: &[Value]) -> Result<Value> {
    match (&args[0], &args[1]) {
        (Value::Int(a), Value::Int(b)) => {
            let mut a = a.abs();
            let mut b = b.abs();
            while b != 0 {
                let t = b;
                b = a % b;
                a = t;
            }
            Ok(Value::Int(a))
        }
        _ => Err(JtvError::TypeError("gcd requires integer arguments".to_string())),
    }
}

fn stdlib_lcm(args: &[Value]) -> Result<Value> {
    match (&args[0], &args[1]) {
        (Value::Int(a), Value::Int(b)) => {
            let gcd_result = stdlib_gcd(args)?;
            if let Value::Int(g) = gcd_result {
                if g == 0 {
                    Ok(Value::Int(0))
                } else {
                    Ok(Value::Int((a.abs() / g) * b.abs()))
                }
            } else {
                unreachable!()
            }
        }
        _ => Err(JtvError::TypeError("lcm requires integer arguments".to_string())),
    }
}

fn stdlib_factorial(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::Int(n) => {
            if *n < 0 {
                return Err(JtvError::RuntimeError("factorial of negative number".to_string()));
            }
            let mut result: i64 = 1;
            for i in 2..=*n {
                result = result.saturating_mul(i);
            }
            Ok(Value::Int(result))
        }
        _ => Err(JtvError::TypeError("factorial requires an integer argument".to_string())),
    }
}

fn stdlib_is_prime(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::Int(n) => {
            if *n <= 1 {
                return Ok(Value::Bool(false));
            }
            if *n <= 3 {
                return Ok(Value::Bool(true));
            }
            if n % 2 == 0 || n % 3 == 0 {
                return Ok(Value::Bool(false));
            }
            let mut i = 5i64;
            while i * i <= *n {
                if n % i == 0 || n % (i + 2) == 0 {
                    return Ok(Value::Bool(false));
                }
                i += 6;
            }
            Ok(Value::Bool(true))
        }
        _ => Err(JtvError::TypeError("isPrime requires an integer argument".to_string())),
    }
}

fn stdlib_pow(args: &[Value]) -> Result<Value> {
    match (&args[0], &args[1]) {
        (Value::Int(base), Value::Int(exp)) => {
            if *exp < 0 {
                return Err(JtvError::RuntimeError("negative exponent for integers".to_string()));
            }
            let mut result: i64 = 1;
            let mut base = *base;
            let mut exp = *exp as u32;
            while exp > 0 {
                if exp & 1 == 1 {
                    result = result.saturating_mul(base);
                }
                base = base.saturating_mul(base);
                exp >>= 1;
            }
            Ok(Value::Int(result))
        }
        (Value::Float(base), Value::Int(exp)) => {
            Ok(Value::Float(base.powi(*exp as i32)))
        }
        (Value::Float(base), Value::Float(exp)) => {
            Ok(Value::Float(base.powf(*exp)))
        }
        _ => Err(JtvError::TypeError("pow requires numeric arguments".to_string())),
    }
}

fn stdlib_sqrt(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::Int(n) => {
            if *n < 0 {
                return Err(JtvError::RuntimeError("sqrt of negative number".to_string()));
            }
            Ok(Value::Float((*n as f64).sqrt()))
        }
        Value::Float(f) => {
            if *f < 0.0 {
                return Err(JtvError::RuntimeError("sqrt of negative number".to_string()));
            }
            Ok(Value::Float(f.sqrt()))
        }
        _ => Err(JtvError::TypeError("sqrt requires a numeric argument".to_string())),
    }
}

fn stdlib_mod(args: &[Value]) -> Result<Value> {
    match (&args[0], &args[1]) {
        (Value::Int(a), Value::Int(b)) => {
            if *b == 0 {
                return Err(JtvError::RuntimeError("modulo by zero".to_string()));
            }
            Ok(Value::Int(a % b))
        }
        _ => Err(JtvError::TypeError("mod requires integer arguments".to_string())),
    }
}

// ===== Collection Functions =====

fn stdlib_length(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::List(items) => Ok(Value::Int(items.len() as i64)),
        Value::Tuple(items) => Ok(Value::Int(items.len() as i64)),
        _ => Err(JtvError::TypeError("length requires a list or tuple".to_string())),
    }
}

fn stdlib_sum(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::List(items) => {
            let mut result = Value::Int(0);
            for item in items {
                result = add_values(&result, item)?;
            }
            Ok(result)
        }
        _ => Err(JtvError::TypeError("sum requires a list".to_string())),
    }
}

fn stdlib_product(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::List(items) => {
            let mut result: i64 = 1;
            for item in items {
                if let Value::Int(n) = item {
                    result = result.saturating_mul(*n);
                } else {
                    return Err(JtvError::TypeError("product requires a list of integers".to_string()));
                }
            }
            Ok(Value::Int(result))
        }
        _ => Err(JtvError::TypeError("product requires a list".to_string())),
    }
}

fn stdlib_head(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::List(items) => {
            items.first()
                .cloned()
                .ok_or_else(|| JtvError::RuntimeError("head of empty list".to_string()))
        }
        _ => Err(JtvError::TypeError("head requires a list".to_string())),
    }
}

fn stdlib_tail(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::List(items) => {
            if items.is_empty() {
                return Err(JtvError::RuntimeError("tail of empty list".to_string()));
            }
            Ok(Value::List(items[1..].to_vec()))
        }
        _ => Err(JtvError::TypeError("tail requires a list".to_string())),
    }
}

fn stdlib_last(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::List(items) => {
            items.last()
                .cloned()
                .ok_or_else(|| JtvError::RuntimeError("last of empty list".to_string()))
        }
        _ => Err(JtvError::TypeError("last requires a list".to_string())),
    }
}

fn stdlib_init(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::List(items) => {
            if items.is_empty() {
                return Err(JtvError::RuntimeError("init of empty list".to_string()));
            }
            Ok(Value::List(items[..items.len()-1].to_vec()))
        }
        _ => Err(JtvError::TypeError("init requires a list".to_string())),
    }
}

fn stdlib_reverse(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::List(items) => {
            let mut reversed = items.clone();
            reversed.reverse();
            Ok(Value::List(reversed))
        }
        _ => Err(JtvError::TypeError("reverse requires a list".to_string())),
    }
}

fn stdlib_range(args: &[Value]) -> Result<Value> {
    match (&args[0], &args[1]) {
        (Value::Int(start), Value::Int(end)) => {
            let list: Vec<Value> = (*start..*end).map(Value::Int).collect();
            Ok(Value::List(list))
        }
        _ => Err(JtvError::TypeError("range requires integer arguments".to_string())),
    }
}

fn stdlib_concat(args: &[Value]) -> Result<Value> {
    match (&args[0], &args[1]) {
        (Value::List(a), Value::List(b)) => {
            let mut result = a.clone();
            result.extend(b.clone());
            Ok(Value::List(result))
        }
        _ => Err(JtvError::TypeError("concat requires two lists".to_string())),
    }
}

fn stdlib_contains(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::List(items) => {
            Ok(Value::Bool(items.contains(&args[1])))
        }
        _ => Err(JtvError::TypeError("contains requires a list".to_string())),
    }
}

fn stdlib_at(args: &[Value]) -> Result<Value> {
    match (&args[0], &args[1]) {
        (Value::List(items), Value::Int(idx)) => {
            let idx = *idx as usize;
            items.get(idx)
                .cloned()
                .ok_or_else(|| JtvError::RuntimeError(format!("index {} out of bounds", idx)))
        }
        (Value::Tuple(items), Value::Int(idx)) => {
            let idx = *idx as usize;
            items.get(idx)
                .cloned()
                .ok_or_else(|| JtvError::RuntimeError(format!("index {} out of bounds", idx)))
        }
        _ => Err(JtvError::TypeError("at requires a list/tuple and integer index".to_string())),
    }
}

// Helper function
fn add_values(a: &Value, b: &Value) -> Result<Value> {
    match (a, b) {
        (Value::Int(x), Value::Int(y)) => Ok(Value::Int(x + y)),
        (Value::Float(x), Value::Float(y)) => Ok(Value::Float(x + y)),
        (Value::Int(x), Value::Float(y)) => Ok(Value::Float(*x as f64 + y)),
        (Value::Float(x), Value::Int(y)) => Ok(Value::Float(x + *y as f64)),
        _ => Err(JtvError::TypeError(format!("Cannot add {:?} and {:?}", a, b))),
    }
}

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
