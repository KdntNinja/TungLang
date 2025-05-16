// Handles Python-like type conversion functions
use crate::value::{BooleanValue, FloatNumber, Integer, StringValue, Value};

pub fn std_int(val: &Value) -> Value {
    match val {
        Value::Integer(n) => Value::Integer(Integer(n.0)),
        Value::FloatNumber(f) => Value::Integer(Integer(f.0 as i64)),
        Value::String(s) => {
            s.0.parse::<i64>()
                .map(|n| Value::Integer(Integer(n)))
                .unwrap_or(Value::Undefined)
        }
        Value::Boolean(BooleanValue(true)) => Value::Integer(Integer(1)),
        Value::Boolean(BooleanValue(false)) => Value::Integer(Integer(0)),
        Value::Array(_) | Value::Dict(_) => Value::Undefined,
        Value::Function { .. } => Value::Undefined,
        Value::Undefined => Value::Undefined,
    }
}

pub fn std_str(val: &Value) -> Value {
    match val {
        Value::String(s) => Value::String(StringValue(s.0.clone())),
        Value::Integer(n) => Value::String(StringValue(n.to_string())),
        Value::FloatNumber(f) => Value::String(StringValue(f.to_string())),
        Value::Boolean(b) => Value::String(StringValue(b.to_string())),
        Value::Undefined => Value::String(StringValue("undefined".to_string())),
        Value::Array(arr) => {
            let items: Vec<String> = arr
                .iter()
                .map(|v| match v {
                    Value::String(s) => format!("\"{}\"", s),
                    _ => format!("{}", v),
                })
                .collect();
            Value::String(StringValue(format!("[{}]", items.join(", "))))
        }
        Value::Dict(map) => {
            let items: Vec<String> = map
                .iter()
                .map(|(k, v)| {
                    let value_str = match v {
                        Value::String(s) => format!("\"{}\"", s),
                        _ => format!("{}", v),
                    };
                    format!("\"{}\": {}", k, value_str)
                })
                .collect();
            Value::String(StringValue(format!("{{{}}}", items.join(", "))))
        }
        Value::Function { .. } => Value::String(StringValue("<function>".to_string())),
    }
}

pub fn std_float(val: &Value) -> Value {
    match val {
        Value::FloatNumber(float_value) => Value::FloatNumber(FloatNumber(float_value.0)),
        Value::Integer(integer_value) => Value::FloatNumber(FloatNumber(integer_value.0 as f64)),
        Value::String(string_value) => string_value
            .0
            .parse::<f64>()
            .map(|float| Value::FloatNumber(FloatNumber(float)))
            .unwrap_or(Value::Undefined),
        Value::Boolean(BooleanValue(true)) => Value::FloatNumber(FloatNumber(1.0)),
        Value::Boolean(BooleanValue(false)) => Value::FloatNumber(FloatNumber(0.0)),
        Value::Function { .. } => Value::Undefined,
        Value::Array(_) | Value::Dict(_) => Value::Undefined,
        Value::Undefined => Value::Undefined,
    }
}

pub fn std_bool(val: &Value) -> Value {
    match val {
        Value::Boolean(boolean_value) => Value::Boolean(BooleanValue(boolean_value.0)),
        Value::Integer(integer_value) => Value::Boolean(BooleanValue(integer_value.0 != 0)),
        Value::FloatNumber(float_value) => Value::Boolean(BooleanValue(float_value.0 != 0.0)),
        Value::String(string_value) => Value::Boolean(BooleanValue(!string_value.0.is_empty())),
        Value::Array(array) => Value::Boolean(BooleanValue(!array.is_empty())),
        Value::Dict(dictionary) => Value::Boolean(BooleanValue(!dictionary.is_empty())),
        Value::Undefined => Value::Boolean(BooleanValue(false)),
        Value::Function { .. } => Value::Boolean(BooleanValue(false)),
    }
}
