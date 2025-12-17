// SPDX-License-Identifier: MIT OR GPL-3.0-or-later OR Palimpsest-0.8
// SPDX-FileCopyrightText: 2025 Julia the Viper Contributors
//
// Common Collections Library - Universal collection operations
// These are language-agnostic and could be shared across implementations

use crate::number::Value;
use crate::error::{JtvError, Result};

/// Get length of a list or tuple
pub fn length(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::List(items) => Ok(Value::Int(items.len() as i64)),
        Value::Tuple(items) => Ok(Value::Int(items.len() as i64)),
        Value::String(s) => Ok(Value::Int(s.len() as i64)),
        _ => Err(JtvError::TypeError("length requires a list, tuple, or string".to_string())),
    }
}

/// Sum all elements in a list
pub fn sum(args: &[Value]) -> Result<Value> {
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

/// Product of all elements in a list
pub fn product(args: &[Value]) -> Result<Value> {
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

/// Get first element of a list
pub fn head(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::List(items) => {
            items.first()
                .cloned()
                .ok_or_else(|| JtvError::RuntimeError("head of empty list".to_string()))
        }
        _ => Err(JtvError::TypeError("head requires a list".to_string())),
    }
}

/// Get all elements except the first
pub fn tail(args: &[Value]) -> Result<Value> {
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

/// Get last element of a list
pub fn last(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::List(items) => {
            items.last()
                .cloned()
                .ok_or_else(|| JtvError::RuntimeError("last of empty list".to_string()))
        }
        _ => Err(JtvError::TypeError("last requires a list".to_string())),
    }
}

/// Get all elements except the last
pub fn init(args: &[Value]) -> Result<Value> {
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

/// Reverse a list
pub fn reverse(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::List(items) => {
            let mut reversed = items.clone();
            reversed.reverse();
            Ok(Value::List(reversed))
        }
        _ => Err(JtvError::TypeError("reverse requires a list".to_string())),
    }
}

/// Create a range of integers [start, end)
pub fn range(args: &[Value]) -> Result<Value> {
    match (&args[0], &args[1]) {
        (Value::Int(start), Value::Int(end)) => {
            let list: Vec<Value> = (*start..*end).map(Value::Int).collect();
            Ok(Value::List(list))
        }
        _ => Err(JtvError::TypeError("range requires integer arguments".to_string())),
    }
}

/// Concatenate two lists
pub fn concat(args: &[Value]) -> Result<Value> {
    match (&args[0], &args[1]) {
        (Value::List(a), Value::List(b)) => {
            let mut result = a.clone();
            result.extend(b.clone());
            Ok(Value::List(result))
        }
        (Value::String(a), Value::String(b)) => {
            Ok(Value::String(format!("{}{}", a, b)))
        }
        _ => Err(JtvError::TypeError("concat requires two lists or two strings".to_string())),
    }
}

/// Check if a list contains a value
pub fn contains(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::List(items) => {
            Ok(Value::Bool(items.contains(&args[1])))
        }
        _ => Err(JtvError::TypeError("contains requires a list".to_string())),
    }
}

/// Get element at index
pub fn at(args: &[Value]) -> Result<Value> {
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

/// Take first n elements
pub fn take(args: &[Value]) -> Result<Value> {
    match (&args[0], &args[1]) {
        (Value::List(items), Value::Int(n)) => {
            let n = (*n as usize).min(items.len());
            Ok(Value::List(items[..n].to_vec()))
        }
        _ => Err(JtvError::TypeError("take requires a list and integer".to_string())),
    }
}

/// Drop first n elements
pub fn drop(args: &[Value]) -> Result<Value> {
    match (&args[0], &args[1]) {
        (Value::List(items), Value::Int(n)) => {
            let n = (*n as usize).min(items.len());
            Ok(Value::List(items[n..].to_vec()))
        }
        _ => Err(JtvError::TypeError("drop requires a list and integer".to_string())),
    }
}

/// Zip two lists together
pub fn zip(args: &[Value]) -> Result<Value> {
    match (&args[0], &args[1]) {
        (Value::List(a), Value::List(b)) => {
            let zipped: Vec<Value> = a.iter().zip(b.iter())
                .map(|(x, y)| Value::Tuple(vec![x.clone(), y.clone()]))
                .collect();
            Ok(Value::List(zipped))
        }
        _ => Err(JtvError::TypeError("zip requires two lists".to_string())),
    }
}

/// Find minimum value in a list
pub fn find_min(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::List(items) => {
            if items.is_empty() {
                return Err(JtvError::RuntimeError("min of empty list".to_string()));
            }
            let mut min_val = items[0].clone();
            for item in items.iter().skip(1) {
                if compare_lt(item, &min_val)? {
                    min_val = item.clone();
                }
            }
            Ok(min_val)
        }
        _ => Err(JtvError::TypeError("findMin requires a list".to_string())),
    }
}

/// Find maximum value in a list
pub fn find_max(args: &[Value]) -> Result<Value> {
    match &args[0] {
        Value::List(items) => {
            if items.is_empty() {
                return Err(JtvError::RuntimeError("max of empty list".to_string()));
            }
            let mut max_val = items[0].clone();
            for item in items.iter().skip(1) {
                if compare_lt(&max_val, item)? {
                    max_val = item.clone();
                }
            }
            Ok(max_val)
        }
        _ => Err(JtvError::TypeError("findMax requires a list".to_string())),
    }
}

// Helper: add two values
fn add_values(a: &Value, b: &Value) -> Result<Value> {
    match (a, b) {
        (Value::Int(x), Value::Int(y)) => Ok(Value::Int(x + y)),
        (Value::Float(x), Value::Float(y)) => Ok(Value::Float(x + y)),
        (Value::Int(x), Value::Float(y)) => Ok(Value::Float(*x as f64 + y)),
        (Value::Float(x), Value::Int(y)) => Ok(Value::Float(x + *y as f64)),
        _ => Err(JtvError::TypeError(format!("Cannot add {:?} and {:?}", a, b))),
    }
}

// Helper: compare less than
fn compare_lt(a: &Value, b: &Value) -> Result<bool> {
    match (a, b) {
        (Value::Int(x), Value::Int(y)) => Ok(x < y),
        (Value::Float(x), Value::Float(y)) => Ok(x < y),
        (Value::Int(x), Value::Float(y)) => Ok((*x as f64) < *y),
        (Value::Float(x), Value::Int(y)) => Ok(*x < (*y as f64)),
        _ => Err(JtvError::TypeError(format!("Cannot compare {:?} and {:?}", a, b))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length() {
        let list = Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(3)]);
        assert_eq!(length(&[list]).unwrap(), Value::Int(3));
    }

    #[test]
    fn test_sum() {
        let list = Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(3)]);
        assert_eq!(sum(&[list]).unwrap(), Value::Int(6));
    }

    #[test]
    fn test_head_tail() {
        let list = Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(3)]);
        assert_eq!(head(&[list.clone()]).unwrap(), Value::Int(1));
        assert_eq!(
            tail(&[list]).unwrap(),
            Value::List(vec![Value::Int(2), Value::Int(3)])
        );
    }

    #[test]
    fn test_range() {
        assert_eq!(
            range(&[Value::Int(1), Value::Int(4)]).unwrap(),
            Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(3)])
        );
    }

    #[test]
    fn test_reverse() {
        let list = Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(3)]);
        assert_eq!(
            reverse(&[list]).unwrap(),
            Value::List(vec![Value::Int(3), Value::Int(2), Value::Int(1)])
        );
    }
}
