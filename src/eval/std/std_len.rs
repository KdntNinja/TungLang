// Handles the TungLang len() built-in function
use crate::value::{Integer, Value};

/// Returns the length of a string, array, or dict
pub fn std_len(val: &Value) -> Value {
    match val {
        Value::String(string_value) => Value::Integer(Integer(string_value.len() as i64)),
        Value::Array(array) => Value::Integer(Integer(array.len() as i64)),
        Value::Dict(dictionary) => Value::Integer(Integer(dictionary.len() as i64)),
        _ => Value::Undefined,
    }
}
