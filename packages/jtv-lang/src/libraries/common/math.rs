// SPDX-License-Identifier: MIT OR GPL-3.0-or-later OR Palimpsest-0.8
// SPDX-FileCopyrightText: 2025 Julia the Viper Contributors
//
// Common Math Library - Universal mathematical functions
// These are language-agnostic and could be shared across implementations

use crate::number::Value;
use crate::error::{JtvError, Result};
use num_traits::Signed;

/// Absolute value
pub fn abs(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::Int(n) => Ok(Value::Int(n.abs())),
        Value::Float(f) => Ok(Value::Float(f.abs())),
        Value::Rational(r) => Ok(Value::Rational(r.abs())),
        _ => Err(JtvError::TypeError("abs requires a numeric argument".to_string())),
    }
}

/// Maximum of two values
pub fn max(args: &[Value]) -> Result<Value> {
    match (&args[0], &args[1]) {
        (Value::Int(a), Value::Int(b)) => Ok(Value::Int(*a.max(b))),
        (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.max(*b))),
        (Value::Int(a), Value::Float(b)) => Ok(Value::Float((*a as f64).max(*b))),
        (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a.max(*b as f64))),
        _ => Err(JtvError::TypeError("max requires numeric arguments".to_string())),
    }
}

/// Minimum of two values
pub fn min(args: &[Value]) -> Result<Value> {
    match (&args[0], &args[1]) {
        (Value::Int(a), Value::Int(b)) => Ok(Value::Int(*a.min(b))),
        (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.min(*b))),
        (Value::Int(a), Value::Float(b)) => Ok(Value::Float((*a as f64).min(*b))),
        (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a.min(*b as f64))),
        _ => Err(JtvError::TypeError("min requires numeric arguments".to_string())),
    }
}

/// Sign of a number (-1, 0, or 1)
pub fn sign(args: &[Value]) -> Result<Value> {
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

/// Clamp value between min and max
pub fn clamp(args: &[Value]) -> Result<Value> {
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

/// Floor - round down to integer
pub fn floor(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::Int(n) => Ok(Value::Int(*n)),
        Value::Float(f) => Ok(Value::Int(f.floor() as i64)),
        _ => Err(JtvError::TypeError("floor requires a numeric argument".to_string())),
    }
}

/// Ceiling - round up to integer
pub fn ceil(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::Int(n) => Ok(Value::Int(*n)),
        Value::Float(f) => Ok(Value::Int(f.ceil() as i64)),
        _ => Err(JtvError::TypeError("ceil requires a numeric argument".to_string())),
    }
}

/// Round to nearest integer
pub fn round(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::Int(n) => Ok(Value::Int(*n)),
        Value::Float(f) => Ok(Value::Int(f.round() as i64)),
        _ => Err(JtvError::TypeError("round requires a numeric argument".to_string())),
    }
}

/// Greatest common divisor (Euclidean algorithm)
pub fn gcd(args: &[Value]) -> Result<Value> {
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

/// Least common multiple
pub fn lcm(args: &[Value]) -> Result<Value> {
    match (&args[0], &args[1]) {
        (Value::Int(a), Value::Int(b)) => {
            let gcd_result = gcd(args)?;
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

/// Factorial
pub fn factorial(args: &[Value]) -> Result<Value> {
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

/// Check if a number is prime
pub fn is_prime(args: &[Value]) -> Result<Value> {
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

/// Power function
pub fn pow(args: &[Value]) -> Result<Value> {
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

/// Square root
pub fn sqrt(args: &[Value]) -> Result<Value> {
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

/// Modulo operation
pub fn modulo(args: &[Value]) -> Result<Value> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abs() {
        assert_eq!(abs(&[Value::Int(-5)]).unwrap(), Value::Int(5));
        assert_eq!(abs(&[Value::Int(5)]).unwrap(), Value::Int(5));
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(&[Value::Int(12), Value::Int(8)]).unwrap(), Value::Int(4));
        assert_eq!(gcd(&[Value::Int(17), Value::Int(13)]).unwrap(), Value::Int(1));
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(&[Value::Int(5)]).unwrap(), Value::Int(120));
        assert_eq!(factorial(&[Value::Int(0)]).unwrap(), Value::Int(1));
    }

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(&[Value::Int(7)]).unwrap(), Value::Bool(true));
        assert_eq!(is_prime(&[Value::Int(8)]).unwrap(), Value::Bool(false));
        assert_eq!(is_prime(&[Value::Int(2)]).unwrap(), Value::Bool(true));
    }
}
