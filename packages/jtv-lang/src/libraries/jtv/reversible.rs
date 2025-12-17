// SPDX-License-Identifier: MIT OR GPL-3.0-or-later OR Palimpsest-0.8
// SPDX-FileCopyrightText: 2025 Julia the Viper Contributors
//
// JtV Reversible Computing Library
// Support functions for JtV's v2 reversible computing features:
// - Inverse operations
// - History tracking
// - Quantum simulation primitives

use crate::number::Value;
use crate::error::{JtvError, Result};

// ===== INVERSE OPERATIONS =====
// These functions provide explicit inverses for addition-only arithmetic

/// Explicit subtraction (inverse of addition)
/// In JtV, subtraction is addition of negation: a - b = a + (-b)
pub fn subtract(args: &[Value]) -> Result<Value> {
    match (&args[0], &args[1]) {
        (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
        (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
        (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 - b)),
        (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a - *b as f64)),
        _ => Err(JtvError::TypeError("subtract requires numeric arguments".to_string())),
    }
}

/// Increment by 1 (commonly used in reversible computing)
pub fn increment(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::Int(n) => Ok(Value::Int(n + 1)),
        Value::Float(f) => Ok(Value::Float(f + 1.0)),
        _ => Err(JtvError::TypeError("increment requires a numeric argument".to_string())),
    }
}

/// Decrement by 1 (inverse of increment)
pub fn decrement(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::Int(n) => Ok(Value::Int(n - 1)),
        Value::Float(f) => Ok(Value::Float(f - 1.0)),
        _ => Err(JtvError::TypeError("decrement requires a numeric argument".to_string())),
    }
}

// ===== IDENTITY AND SWAP =====

/// Identity function (always reversible)
pub fn identity(args: &[Value]) -> Result<Value> {
    Ok(args[0].clone())
}

/// Swap two values (self-inverse operation)
pub fn swap(args: &[Value]) -> Result<Value> {
    Ok(Value::Tuple(vec![args[1].clone(), args[0].clone()]))
}

/// Controlled NOT (CNOT) - quantum-inspired operation
/// If control is truthy, negate the target
pub fn cnot(args: &[Value]) -> Result<Value> {
    let control = is_truthy(&args[0]);
    match &args[1] {
        Value::Int(n) => {
            if control {
                Ok(Value::Int(-n))
            } else {
                Ok(Value::Int(*n))
            }
        }
        Value::Bool(b) => {
            if control {
                Ok(Value::Bool(!b))
            } else {
                Ok(Value::Bool(*b))
            }
        }
        _ => Err(JtvError::TypeError("cnot target must be Int or Bool".to_string())),
    }
}

// ===== REVERSIBILITY HELPERS =====

/// Check if a value can be inverted
pub fn is_invertible(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::Int(_) | Value::Float(_) | Value::Rational(_) | Value::Complex(_) => {
            Ok(Value::Bool(true))
        }
        Value::Bool(_) => Ok(Value::Bool(true)),
        _ => Ok(Value::Bool(false)),
    }
}

/// Get the additive inverse of a value
pub fn additive_inverse(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::Int(n) => Ok(Value::Int(-n)),
        Value::Float(f) => Ok(Value::Float(-f)),
        Value::Rational(r) => Ok(Value::Rational(-r)),
        Value::Complex(c) => Ok(Value::Complex(-c)),
        _ => Err(JtvError::TypeError("additive_inverse requires a numeric value".to_string())),
    }
}

/// XOR operation (self-inverse, useful in reversible computing)
pub fn xor(args: &[Value]) -> Result<Value> {
    match (&args[0], &args[1]) {
        (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a ^ b)),
        (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(*a ^ *b)),
        _ => Err(JtvError::TypeError("xor requires integer or bool arguments".to_string())),
    }
}

// ===== HISTORY TRACKING =====
// For reversible execution, we may need to track operation history

/// Create a history entry for a value change
pub fn make_history_entry(args: &[Value]) -> Result<Value> {
    // Returns tuple: (old_value, new_value, operation_name)
    Ok(Value::Tuple(vec![
        args[0].clone(),  // old value
        args[1].clone(),  // new value
        args.get(2).cloned().unwrap_or(Value::String("unknown".to_string())),
    ]))
}

/// Extract old value from history entry
pub fn history_old(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::Tuple(items) if items.len() >= 2 => Ok(items[0].clone()),
        _ => Err(JtvError::TypeError("history_old requires a history tuple".to_string())),
    }
}

/// Extract new value from history entry
pub fn history_new(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::Tuple(items) if items.len() >= 2 => Ok(items[1].clone()),
        _ => Err(JtvError::TypeError("history_new requires a history tuple".to_string())),
    }
}

// ===== QUANTUM SIMULATION PRIMITIVES =====
// Basic building blocks for quantum algorithm simulation

/// Hadamard-like transformation on boolean
/// Represents superposition (returns both possibilities)
pub fn hadamard_bool(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::Bool(_) => {
            // Returns both possible outcomes as a list
            Ok(Value::List(vec![Value::Bool(true), Value::Bool(false)]))
        }
        _ => Err(JtvError::TypeError("hadamard_bool requires a Bool".to_string())),
    }
}

/// Measure a superposition (collapse to single value)
/// Takes first element (deterministic for now)
pub fn measure(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::List(items) if !items.is_empty() => Ok(items[0].clone()),
        _ => Err(JtvError::TypeError("measure requires a non-empty list".to_string())),
    }
}

/// Phase rotation (for quantum phase estimation simulation)
pub fn phase_rotate(args: &[Value]) -> Result<Value> {
    match (&args[0], &args[1]) {
        (Value::Complex(c), Value::Float(angle)) => {
            use std::f64::consts::PI;
            let phase = num_complex::Complex64::from_polar(1.0, angle * PI);
            Ok(Value::Complex(c * phase))
        }
        (Value::Float(f), Value::Float(angle)) => {
            use std::f64::consts::PI;
            let c = num_complex::Complex64::new(*f, 0.0);
            let phase = num_complex::Complex64::from_polar(1.0, angle * PI);
            Ok(Value::Complex(c * phase))
        }
        _ => Err(JtvError::TypeError("phase_rotate requires (Complex, Float)".to_string())),
    }
}

// ===== THERMODYNAMIC EFFICIENCY =====
// Landauer's principle: reversible computing = thermodynamically efficient

/// Calculate information content (entropy proxy)
pub fn bit_count(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::Int(n) => {
            let bits = if *n == 0 { 0 } else { (64 - n.abs().leading_zeros()) as i64 };
            Ok(Value::Int(bits))
        }
        Value::List(items) => {
            Ok(Value::Int(items.len() as i64))
        }
        _ => Err(JtvError::TypeError("bit_count requires Int or List".to_string())),
    }
}

// Helper function
fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Bool(b) => *b,
        Value::Int(n) => *n != 0,
        Value::Float(f) => *f != 0.0,
        Value::Unit => false,
        Value::List(items) => !items.is_empty(),
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(&[Value::Int(10), Value::Int(3)]).unwrap(), Value::Int(7));
    }

    #[test]
    fn test_swap() {
        let result = swap(&[Value::Int(1), Value::Int(2)]).unwrap();
        assert_eq!(result, Value::Tuple(vec![Value::Int(2), Value::Int(1)]));
    }

    #[test]
    fn test_xor_self_inverse() {
        // XOR is its own inverse: a ^ b ^ b = a
        let a = Value::Int(42);
        let b = Value::Int(17);
        let xored = xor(&[a.clone(), b.clone()]).unwrap();
        let restored = xor(&[xored, b]).unwrap();
        assert_eq!(restored, a);
    }

    #[test]
    fn test_additive_inverse() {
        let inv = additive_inverse(&[Value::Int(5)]).unwrap();
        assert_eq!(inv, Value::Int(-5));
    }

    #[test]
    fn test_cnot() {
        // Control false -> no change
        assert_eq!(cnot(&[Value::Bool(false), Value::Int(5)]).unwrap(), Value::Int(5));
        // Control true -> negate
        assert_eq!(cnot(&[Value::Bool(true), Value::Int(5)]).unwrap(), Value::Int(-5));
    }
}
