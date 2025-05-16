// Handles the TungLang len() built-in function
use crate::value::{Value, Number, StringValue};

/// Returns the length of a string, array, or dict
pub fn std_len(val: &Value) -> Value {
    match val {
        Value::String(s) => Value::Number(Number(s.len() as i64)),
        Value::Array(arr) => Value::Number(Number(arr.len() as i64)),
        Value::Dict(map) => Value::Number(Number(map.len() as i64)),
        _ => Value::Undefined,
    }
}
