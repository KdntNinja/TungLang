// Handles the TungLang abs() built-in function
use crate::value::{Value, Number, Float};

/// Returns the absolute value of a number or float
pub fn std_abs(val: &Value) -> Value {
    match val {
        Value::Number(Number(n)) => Value::Number(Number(n.abs())),
        Value::Float(Float(f)) => Value::Float(Float(f.abs())),
        _ => Value::Undefined,
    }
}
