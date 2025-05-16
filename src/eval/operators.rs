// Handles arithmetic and logical operators for TungLang
use crate::value::Value;
use miette::Result;

/// Applies a binary operator to two Values
pub fn apply_operator(left: Value, right: Value, op: &str) -> Result<Value> {
    use crate::value::{BooleanValue, Float, Integer, StringValue};
    match (left.clone(), right.clone(), op) {
        // Arithmetic - Python-like behavior with auto-promotion to float
        (Value::Integer(l), Value::Integer(r), "+") => Ok(Value::Integer(l + r)),
        (Value::Integer(l), Value::Integer(r), "-") => Ok(Value::Integer(l - r)),
        (Value::Integer(l), Value::Integer(r), "*") => Ok(Value::Integer(l * r)),
        (Value::Integer(l), Value::Integer(r), "/") => {
            Ok(Value::Float(Float(l.0 as f64 / r.0 as f64)))
        } // Division always returns float in Python
        (Value::Integer(l), Value::Integer(r), "//") => Ok(Value::Integer(Integer(l.0 / r.0))), // Floor division
        (Value::Integer(l), Value::Integer(r), "%") => Ok(Value::Integer(Integer(l.0 % r.0))),
        (Value::Integer(l), Value::Integer(r), "**") => {
            Ok(Value::Float(Float((l.0 as f64).powf(r.0 as f64))))
        } // Exponentiation

        // Mixed number and float operations (auto-promotion)
        (Value::Integer(l), Value::Float(r), "+") => Ok(Value::Float(Float(l.0 as f64 + r.0))),
        (Value::Integer(l), Value::Float(r), "-") => Ok(Value::Float(Float(l.0 as f64 - r.0))),
        (Value::Integer(l), Value::Float(r), "*") => Ok(Value::Float(Float(l.0 as f64 * r.0))),
        (Value::Integer(l), Value::Float(r), "/") => Ok(Value::Float(Float(l.0 as f64 / r.0))),
        (Value::Integer(l), Value::Float(r), "//") => {
            Ok(Value::Integer(Integer((l.0 as f64 / r.0).floor() as i64)))
        }
        (Value::Integer(l), Value::Float(r), "%") => Ok(Value::Float(Float((l.0 as f64) % r.0))),
        (Value::Integer(l), Value::Float(r), "**") => {
            Ok(Value::Float(Float((l.0 as f64).powf(r.0))))
        }

        (Value::Float(l), Value::Integer(r), "+") => Ok(Value::Float(Float(l.0 + r.0 as f64))),
        (Value::Float(l), Value::Integer(r), "-") => Ok(Value::Float(Float(l.0 - r.0 as f64))),
        (Value::Float(l), Value::Integer(r), "*") => Ok(Value::Float(Float(l.0 * r.0 as f64))),
        (Value::Float(l), Value::Integer(r), "/") => Ok(Value::Float(Float(l.0 / r.0 as f64))),
        (Value::Float(l), Value::Integer(r), "//") => {
            Ok(Value::Integer(Integer((l.0 / r.0 as f64).floor() as i64)))
        }
        (Value::Float(l), Value::Integer(r), "%") => Ok(Value::Float(Float(l.0 % r.0 as f64))),
        (Value::Float(l), Value::Integer(r), "**") => Ok(Value::Float(Float(l.0.powf(r.0 as f64)))),

        (Value::Float(l), Value::Float(r), "+") => Ok(Value::Float(Float(l.0 + r.0))),
        (Value::Float(l), Value::Float(r), "-") => Ok(Value::Float(Float(l.0 - r.0))),
        (Value::Float(l), Value::Float(r), "*") => Ok(Value::Float(Float(l.0 * r.0))),
        (Value::Float(l), Value::Float(r), "/") => Ok(Value::Float(Float(l.0 / r.0))),
        (Value::Float(l), Value::Float(r), "//") => {
            Ok(Value::Integer(Integer((l.0 / r.0).floor() as i64)))
        }
        (Value::Float(l), Value::Float(r), "%") => Ok(Value::Float(Float(l.0 % r.0))),
        (Value::Float(l), Value::Float(r), "**") => Ok(Value::Float(Float(l.0.powf(r.0)))),
        // String concatenation and Python-like string operations
        (Value::String(l), Value::String(r), "+") => Ok(Value::String(l + &r)),
        (Value::String(l), Value::Integer(r), "+") => {
            Ok(Value::String(l + &StringValue(r.0.to_string())))
        }
        (Value::String(l), Value::Float(r), "+") => {
            Ok(Value::String(l + &StringValue(r.0.to_string())))
        }
        (Value::String(l), Value::Boolean(r), "+") => {
            Ok(Value::String(l + &StringValue(r.0.to_string())))
        }
        (Value::String(l), Value::Array(r), "+") => {
            Ok(Value::String(l + &StringValue(format!("{:?}", r))))
        }
        (Value::String(l), Value::Dict(r), "+") => {
            Ok(Value::String(l + &StringValue(format!("{:?}", r))))
        }
        (Value::String(l), Value::Undefined, "+") => {
            Ok(Value::String(l + &StringValue("undefined".to_string())))
        }

        (Value::Integer(l), Value::String(r), "+") => {
            Ok(Value::String(StringValue(l.0.to_string()) + &r))
        }
        (Value::Float(l), Value::String(r), "+") => {
            Ok(Value::String(StringValue(l.0.to_string()) + &r))
        }
        (Value::Boolean(l), Value::String(r), "+") => {
            Ok(Value::String(StringValue(l.0.to_string()) + &r))
        }
        (Value::Array(l), Value::String(r), "+") => {
            Ok(Value::String(StringValue(format!("{:?}", l)) + &r))
        }
        (Value::Dict(l), Value::String(r), "+") => {
            Ok(Value::String(StringValue(format!("{:?}", l)) + &r))
        }
        (Value::Undefined, Value::String(r), "+") => {
            Ok(Value::String(StringValue("undefined".to_string()) + &r))
        }

        // Python-like string repetition with * operator
        (Value::String(s), Value::Integer(n), "*") => {
            if n.0 <= 0 {
                Ok(Value::String(StringValue("".to_string())))
            } else {
                Ok(Value::String(s.repeat(n.0 as usize)))
            }
        }
        (Value::Integer(n), Value::String(s), "*") => {
            if n.0 <= 0 {
                Ok(Value::String(StringValue("".to_string())))
            } else {
                Ok(Value::String(s.repeat(n.0 as usize)))
            }
        }
        // Array concatenation and other Python-like array operations
        (Value::Array(mut l), Value::Array(r), "+") => {
            l.extend(r);
            Ok(Value::Array(l))
        }
        // Python-like array/item concatenation (adding an item to array)
        (Value::Array(mut l), right, "+") => {
            l.push(right);
            Ok(Value::Array(l))
        }
        (left, Value::Array(mut r), "+") => {
            r.insert(0, left);
            Ok(Value::Array(r))
        }
        // Python-like array multiplication (repeat arrays)
        (Value::Array(a), Value::Integer(n), "*") => {
            if n.0 <= 0 {
                Ok(Value::Array(vec![]))
            } else {
                let mut result = Vec::new();
                for _ in 0..n.0 {
                    result.extend(a.clone());
                }
                Ok(Value::Array(result))
            }
        }
        (Value::Integer(n), Value::Array(a), "*") => {
            if n.0 <= 0 {
                Ok(Value::Array(vec![]))
            } else {
                let mut result = Vec::new();
                for _ in 0..n.0 {
                    result.extend(a.clone());
                }
                Ok(Value::Array(result))
            }
        }
        // Equality
        (Value::Integer(l), Value::Integer(r), "==") => Ok(Value::Boolean(BooleanValue(l == r))),
        (Value::Float(l), Value::Float(r), "==") => Ok(Value::Boolean(BooleanValue(l == r))),
        (Value::String(l), Value::String(r), "==") => Ok(Value::Boolean(BooleanValue(l == r))),
        (Value::Boolean(l), Value::Boolean(r), "==") => Ok(Value::Boolean(BooleanValue(l == r))),
        // Inequality
        (Value::Integer(l), Value::Integer(r), "!=") => Ok(Value::Boolean(BooleanValue(l != r))),
        (Value::Float(l), Value::Float(r), "!=") => Ok(Value::Boolean(BooleanValue(l != r))),
        (Value::String(l), Value::String(r), "!=") => Ok(Value::Boolean(BooleanValue(l != r))),
        (Value::Boolean(l), Value::Boolean(r), "!=") => Ok(Value::Boolean(BooleanValue(l != r))),
        // Comparison
        (Value::Integer(l), Value::Integer(r), op) if matches!(op, ">" | "<" | ">=" | "<=") => {
            let res = match op {
                ">" => l > r,
                "<" => l < r,
                ">=" => l >= r,
                "<=" => l <= r,
                _ => unreachable!(),
            };
            Ok(Value::Boolean(BooleanValue(res)))
        }
        (Value::Float(l), Value::Float(r), op) if matches!(op, ">" | "<" | ">=" | "<=") => {
            let res = match op {
                ">" => l > r,
                "<" => l < r,
                ">=" => l >= r,
                "<=" => l <= r,
                _ => unreachable!(),
            };
            Ok(Value::Boolean(BooleanValue(res)))
        }
        (Value::String(l), Value::String(r), op) if matches!(op, ">" | "<" | ">=" | "<=") => {
            let res = match op {
                ">" => l > r,
                "<" => l < r,
                ">=" => l >= r,
                "<=" => l <= r,
                _ => unreachable!(),
            };
            Ok(Value::Boolean(BooleanValue(res)))
        }
        // Logical
        (Value::Boolean(l), Value::Boolean(r), "&&") => {
            Ok(Value::Boolean(BooleanValue(l.0 && r.0)))
        }
        (Value::Boolean(l), Value::Boolean(r), "||") => {
            Ok(Value::Boolean(BooleanValue(l.0 || r.0)))
        }
        // Unary
        (Value::Boolean(l), Value::Undefined, "!") => Ok(Value::Boolean(!l)),
        (Value::Integer(l), Value::Undefined, "-") => Ok(Value::Integer(-l)),
        (Value::Float(l), Value::Undefined, "-") => Ok(Value::Float(-l)),
        // Type conversion for comparison (Python allows comparing different numeric types)
        (Value::Integer(l), Value::Float(r), op)
            if matches!(op, "==" | "!=" | ">" | "<" | ">=" | "<=") =>
        {
            let left = l.0 as f64;
            let result = match op {
                "==" => left == r.0,
                "!=" => left != r.0,
                ">" => left > r.0,
                "<" => left < r.0,
                ">=" => left >= r.0,
                "<=" => left <= r.0,
                _ => unreachable!(),
            };
            Ok(Value::Boolean(BooleanValue(result)))
        }
        (Value::Float(l), Value::Integer(r), op)
            if matches!(op, "==" | "!=" | ">" | "<" | ">=" | "<=") =>
        {
            let right = r.0 as f64;
            let result = match op {
                "==" => l.0 == right,
                "!=" => l.0 != right,
                ">" => l.0 > right,
                "<" => l.0 < right,
                ">=" => l.0 >= right,
                "<=" => l.0 <= right,
                _ => unreachable!(),
            };
            Ok(Value::Boolean(BooleanValue(result)))
        }

        // Python-like 'in' operator for arrays and dicts
        (item, Value::Array(arr), "in") => Ok(Value::Boolean(BooleanValue(arr.contains(&item)))),
        (Value::String(key), Value::Dict(dict), "in") => {
            Ok(Value::Boolean(BooleanValue(dict.contains_key(&key.0))))
        }

        // Python-like 'not in' operator for arrays and dicts
        (item, Value::Array(arr), "!in") => Ok(Value::Boolean(BooleanValue(!arr.contains(&item)))),
        (Value::String(key), Value::Dict(dict), "!in") => {
            Ok(Value::Boolean(BooleanValue(!dict.contains_key(&key.0))))
        }

        // Fall through cases
        _ => Err(miette::miette!(
            "Error: Unsupported operation '{}' between types {:?} and {:?}",
            op,
            left,
            right
        )),
    }
}
