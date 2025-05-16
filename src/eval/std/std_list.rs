// file: /home/kaiden/RustroverProjects/TungLang/src/eval/std_list.rs
// Python-like list functions for TungLang
use crate::value::{Value, Number, Float, StringValue, BooleanValue};

// append function (modifies list in-place like Python's list.append())
pub fn std_append(args: &[Value]) -> Value {
    if args.len() < 2 {
        return Value::Undefined;
    }

    match args[0].clone() {
        Value::Array(mut arr) => {
            arr.push(args[1].clone());
            Value::Array(arr)
        }
        _ => Value::Undefined,
    }
}

// insert function (modifies list in-place like Python's list.insert())
pub fn std_insert(args: &[Value]) -> Value {
    if args.len() < 3 {
        return Value::Undefined;
    }

    match (args[0].clone(), &args[1]) {
        (Value::Array(mut arr), Value::Number(Number(idx))) => {
            let index = if *idx < 0 {
                arr.len().saturating_sub(idx.unsigned_abs() as usize)
            } else {
                *idx as usize
            };

            let clamped_index = index.min(arr.len());
            arr.insert(clamped_index, args[2].clone());
            Value::Array(arr)
        }
        _ => Value::Undefined,
    }
}

// pop function (removes and returns item at index, default is last)
pub fn std_pop(args: &[Value]) -> Value {
    if args.is_empty() {
        return Value::Undefined;
    }

    match args[0].clone() {
        Value::Array(mut arr) => {
            if arr.is_empty() {
                return Value::Undefined;
            }

            let idx = if args.len() > 1 {
                match args[1] {
                    Value::Number(Number(n)) => {
                        if n < 0 {
                            arr.len().saturating_sub(n.unsigned_abs() as usize)
                        } else {
                            n as usize
                        }
                    }
                    _ => arr.len() - 1,
                }
            } else {
                arr.len() - 1
            };

            if idx < arr.len() {
                let removed = arr.remove(idx);
                removed
            } else {
                Value::Undefined
            }
        }
        _ => Value::Undefined,
    }
}

// index function (returns the index of the first occurrence of value)
pub fn std_index(args: &[Value]) -> Value {
    if args.len() < 2 {
        return Value::Undefined;
    }

    match &args[0] {
        Value::Array(arr) => {
            for (i, item) in arr.iter().enumerate() {
                if item == &args[1] {
                    return Value::Number(Number(i as i64));
                }
            }
            Value::Number(Number(-1))
        }
        Value::String(s) => {
            if let Value::String(substr) = &args[1] {
                match s.0.find(&substr.0) {
                    Some(idx) => Value::Number(Number(idx as i64)),
                    None => Value::Number(Number(-1)),
                }
            } else {
                Value::Number(Number(-1))
            }
        }
        _ => Value::Undefined,
    }
}

// sort function (sorts a list in-place)
pub fn std_sort(args: &[Value]) -> Value {
    if args.is_empty() {
        return Value::Undefined;
    }

    match args[0].clone() {
        Value::Array(mut arr) => {
            arr.sort_by(|a, b| match (a, b) {
                (Value::Number(Number(n1)), Value::Number(Number(n2))) => n1.cmp(n2),
                (Value::Float(Float(f1)), Value::Float(Float(f2))) => {
                    f1.partial_cmp(f2).unwrap_or(std::cmp::Ordering::Equal)
                }
                (Value::Number(Number(n)), Value::Float(Float(f))) => (*n as f64)
                    .partial_cmp(f)
                    .unwrap_or(std::cmp::Ordering::Equal),
                (Value::Float(Float(f)), Value::Number(Number(n))) => f
                    .partial_cmp(&(*n as f64))
                    .unwrap_or(std::cmp::Ordering::Equal),
                (Value::String(s1), Value::String(s2)) => s1.cmp(s2),
                _ => std::cmp::Ordering::Equal,
            });
            Value::Array(arr)
        }
        _ => Value::Undefined,
    }
}
