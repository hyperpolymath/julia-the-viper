// SPDX-License-Identifier: MIT OR GPL-3.0-or-later OR Palimpsest-0.8
// SPDX-FileCopyrightText: 2025 Julia the Viper Contributors
//
// JtV Number Systems Library
// Functions for working with JtV's 7 number systems:
// Int, Float, Rational, Complex, Hex, Binary, Symbolic

use crate::number::Value;
use crate::error::{JtvError, Result};
use num_rational::Ratio;
use num_complex::Complex64;

// ===== RATIONAL NUMBER OPERATIONS =====

/// Create a rational number from numerator and denominator
pub fn rational(args: &[Value]) -> Result<Value> {
    match (&args[0], &args[1]) {
        (Value::Int(num), Value::Int(den)) => {
            if *den == 0 {
                return Err(JtvError::DivisionByZero);
            }
            Ok(Value::Rational(Ratio::new(*num, *den)))
        }
        _ => Err(JtvError::TypeError("rational requires two integers".to_string())),
    }
}

/// Get numerator of a rational
pub fn numerator(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::Rational(r) => Ok(Value::Int(*r.numer())),
        Value::Int(n) => Ok(Value::Int(*n)),
        _ => Err(JtvError::TypeError("numerator requires a rational".to_string())),
    }
}

/// Get denominator of a rational
pub fn denominator(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::Rational(r) => Ok(Value::Int(*r.denom())),
        Value::Int(_) => Ok(Value::Int(1)),
        _ => Err(JtvError::TypeError("denominator requires a rational".to_string())),
    }
}

/// Convert to reduced form
pub fn reduce(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::Rational(r) => Ok(Value::Rational(r.reduced())),
        other => Ok(other.clone()),
    }
}

// ===== COMPLEX NUMBER OPERATIONS =====

/// Create a complex number from real and imaginary parts
pub fn complex(args: &[Value]) -> Result<Value> {
    let real = match &args[0] {
        Value::Int(n) => *n as f64,
        Value::Float(f) => *f,
        _ => return Err(JtvError::TypeError("complex requires numeric arguments".to_string())),
    };
    let imag = match &args[1] {
        Value::Int(n) => *n as f64,
        Value::Float(f) => *f,
        _ => return Err(JtvError::TypeError("complex requires numeric arguments".to_string())),
    };
    Ok(Value::Complex(Complex64::new(real, imag)))
}

/// Get real part of a complex number
pub fn real_part(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::Complex(c) => Ok(Value::Float(c.re)),
        Value::Float(f) => Ok(Value::Float(*f)),
        Value::Int(n) => Ok(Value::Float(*n as f64)),
        _ => Err(JtvError::TypeError("realPart requires a complex number".to_string())),
    }
}

/// Get imaginary part of a complex number
pub fn imag_part(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::Complex(c) => Ok(Value::Float(c.im)),
        Value::Float(_) | Value::Int(_) => Ok(Value::Float(0.0)),
        _ => Err(JtvError::TypeError("imagPart requires a complex number".to_string())),
    }
}

/// Get magnitude (absolute value) of a complex number
pub fn magnitude(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::Complex(c) => Ok(Value::Float(c.norm())),
        Value::Float(f) => Ok(Value::Float(f.abs())),
        Value::Int(n) => Ok(Value::Float((*n as f64).abs())),
        _ => Err(JtvError::TypeError("magnitude requires a numeric argument".to_string())),
    }
}

/// Get phase angle of a complex number
pub fn phase(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::Complex(c) => Ok(Value::Float(c.arg())),
        _ => Err(JtvError::TypeError("phase requires a complex number".to_string())),
    }
}

/// Complex conjugate
pub fn conjugate(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::Complex(c) => Ok(Value::Complex(c.conj())),
        other => Ok(other.clone()), // Real numbers are their own conjugate
    }
}

// ===== HEX AND BINARY OPERATIONS =====

/// Convert integer to hex representation
pub fn to_hex(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::Int(n) => Ok(Value::Hex(*n)),
        Value::Hex(n) => Ok(Value::Hex(*n)),
        Value::Binary(n) => Ok(Value::Hex(*n)),
        _ => Err(JtvError::TypeError("toHex requires an integer".to_string())),
    }
}

/// Convert integer to binary representation
pub fn to_binary(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::Int(n) => Ok(Value::Binary(*n)),
        Value::Hex(n) => Ok(Value::Binary(*n)),
        Value::Binary(n) => Ok(Value::Binary(*n)),
        _ => Err(JtvError::TypeError("toBinary requires an integer".to_string())),
    }
}

/// Convert any numeric to integer
pub fn to_int(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::Int(n) => Ok(Value::Int(*n)),
        Value::Float(f) => Ok(Value::Int(*f as i64)),
        Value::Hex(n) => Ok(Value::Int(*n)),
        Value::Binary(n) => Ok(Value::Int(*n)),
        Value::Rational(r) => Ok(Value::Int((*r.numer()) / (*r.denom()))),
        _ => Err(JtvError::TypeError("toInt requires a numeric value".to_string())),
    }
}

/// Convert any numeric to float
pub fn to_float(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::Int(n) => Ok(Value::Float(*n as f64)),
        Value::Float(f) => Ok(Value::Float(*f)),
        Value::Hex(n) => Ok(Value::Float(*n as f64)),
        Value::Binary(n) => Ok(Value::Float(*n as f64)),
        Value::Rational(r) => Ok(Value::Float(*r.numer() as f64 / *r.denom() as f64)),
        _ => Err(JtvError::TypeError("toFloat requires a numeric value".to_string())),
    }
}

// ===== SYMBOLIC OPERATIONS =====

/// Create a symbolic expression
pub fn symbolic(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::String(s) => Ok(Value::Symbolic(s.clone())),
        Value::Symbolic(s) => Ok(Value::Symbolic(s.clone())),
        other => Ok(Value::Symbolic(format!("{}", other))),
    }
}

/// Check if value is symbolic
pub fn is_symbolic(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::Symbolic(_) => Ok(Value::Bool(true)),
        _ => Ok(Value::Bool(false)),
    }
}

// ===== TYPE CHECKING =====

/// Check if value is a rational number
pub fn is_rational(args: &[Value]) -> Result<Value> {
    Ok(Value::Bool(matches!(&args[0], Value::Rational(_))))
}

/// Check if value is a complex number
pub fn is_complex(args: &[Value]) -> Result<Value> {
    Ok(Value::Bool(matches!(&args[0], Value::Complex(_))))
}

/// Check if value is an integer
pub fn is_int(args: &[Value]) -> Result<Value> {
    Ok(Value::Bool(matches!(&args[0], Value::Int(_))))
}

/// Check if value is a float
pub fn is_float(args: &[Value]) -> Result<Value> {
    Ok(Value::Bool(matches!(&args[0], Value::Float(_))))
}

/// Get type name as string
pub fn type_of(args: &[Value]) -> Result<Value> {
    let type_name = match &args[0] {
        Value::Int(_) => "Int",
        Value::Float(_) => "Float",
        Value::Rational(_) => "Rational",
        Value::Complex(_) => "Complex",
        Value::Hex(_) => "Hex",
        Value::Binary(_) => "Binary",
        Value::Symbolic(_) => "Symbolic",
        Value::Bool(_) => "Bool",
        Value::String(_) => "String",
        Value::List(_) => "List",
        Value::Tuple(_) => "Tuple",
        Value::Unit => "Unit",
    };
    Ok(Value::String(type_name.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rational() {
        let r = rational(&[Value::Int(3), Value::Int(4)]).unwrap();
        assert!(matches!(r, Value::Rational(_)));
    }

    #[test]
    fn test_complex() {
        let c = complex(&[Value::Float(3.0), Value::Float(4.0)]).unwrap();
        if let Value::Complex(c) = c {
            assert_eq!(c.re, 3.0);
            assert_eq!(c.im, 4.0);
        } else {
            panic!("Expected Complex");
        }
    }

    #[test]
    fn test_magnitude() {
        let c = Value::Complex(Complex64::new(3.0, 4.0));
        let mag = magnitude(&[c]).unwrap();
        if let Value::Float(m) = mag {
            assert!((m - 5.0).abs() < 0.0001);
        } else {
            panic!("Expected Float");
        }
    }

    #[test]
    fn test_type_of() {
        assert_eq!(type_of(&[Value::Int(5)]).unwrap(), Value::String("Int".to_string()));
        assert_eq!(type_of(&[Value::Float(3.14)]).unwrap(), Value::String("Float".to_string()));
    }
}
