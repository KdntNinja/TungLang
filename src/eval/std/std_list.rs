// file: /home/kaiden/RustroverProjects/TungLang/src/eval/std_list.rs
// Python-like list functions for TungLang
use crate::value::{BooleanValue, FloatNumber, Integer, StringValue, Value};

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
        (Value::Array(mut arr), Value::Integer(Integer(idx))) => {
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
        Value::Array(mut array) => {
            if array.is_empty() {
                return Value::Undefined;
            }

            let idx = if args.len() > 1 {
                match &args[1] {
                    Value::Integer(Integer(index_value)) => {
                        if *index_value < 0 {
                            array
                                .len()
                                .saturating_sub(index_value.unsigned_abs() as usize)
                        } else {
                            *index_value as usize
                        }
                    }
                    _ => array.len() - 1,
                }
            } else {
                array.len() - 1
            };

            if idx < array.len() {
                let removed = array.remove(idx);
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
        Value::Array(array) => {
            for (i, item) in array.iter().enumerate() {
                if item == &args[1] {
                    return Value::Integer(Integer(i as i64));
                }
            }
            Value::Integer(Integer(-1))
        }
        Value::String(string_value) => {
            if let Value::String(substring_value) = &args[1] {
                match string_value.0.find(&substring_value.0) {
                    Some(idx) => Value::Integer(Integer(idx as i64)),
                    _ => Value::Integer(Integer(-1)),
                }
            } else {
                Value::Integer(Integer(-1))
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
        Value::Array(mut array) => {
            array.sort_by(|a, b| match (a, b) {
                (Value::Integer(Integer(n1)), Value::Integer(Integer(n2))) => n1.cmp(n2),
                (Value::FloatNumber(FloatNumber(f1)), Value::FloatNumber(FloatNumber(f2))) => {
                    f1.partial_cmp(f2).unwrap_or(std::cmp::Ordering::Equal)
                }
                (Value::Integer(Integer(n)), Value::FloatNumber(FloatNumber(f))) => (*n as f64)
                    .partial_cmp(f)
                    .unwrap_or(std::cmp::Ordering::Equal),
                (Value::FloatNumber(FloatNumber(f)), Value::Integer(Integer(n))) => f
                    .partial_cmp(&(*n as f64))
                    .unwrap_or(std::cmp::Ordering::Equal),
                (Value::String(string1), Value::String(string2)) => string1.cmp(string2),
                _ => std::cmp::Ordering::Equal,
            });
            Value::Array(array)
        }
        _ => Value::Undefined,
    }
}
