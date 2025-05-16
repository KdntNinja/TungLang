// Handles the TungLang abs() built-in function
use crate::value::{FloatNumber, Integer, Value};

/// Returns the absolute value of a number or float
pub fn std_abs(val: &Value) -> Value {
    match val {
        Value::Integer(Integer(integer_value)) => Value::Integer(Integer(integer_value.abs())),
        Value::FloatNumber(FloatNumber(float_value)) => {
            Value::FloatNumber(FloatNumber(float_value.abs()))
        }
        _ => Value::Undefined,
    }
}
