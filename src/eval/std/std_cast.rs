// Handles Python-like type conversion functions
use crate::value::{Value, Number, Float, StringValue, BooleanValue};

pub fn std_int(val: &Value) -> Value {
    match val {
        Value::Number(n) => Value::Number(Number(n.0)),
        Value::Float(f) => Value::Number(Number(f.0 as i64)),
        Value::String(s) => s.0
            .parse::<i64>()
            .map(|n| Value::Number(Number(n)))
            .unwrap_or(Value::Undefined),
        Value::Boolean(BooleanValue(true)) => Value::Number(Number(1)),
        Value::Boolean(BooleanValue(false)) => Value::Number(Number(0)),
        Value::Array(_) | Value::Dict(_) => Value::Undefined,
        Value::Function { .. } => Value::Undefined,
        Value::Undefined => Value::Undefined,
    }
}

pub fn std_str(val: &Value) -> Value {
    match val {
        Value::String(s) => Value::String(StringValue(s.0.clone())),
        Value::Number(n) => Value::String(StringValue(n.to_string())),
        Value::Float(f) => Value::String(StringValue(f.to_string())),
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
        Value::Float(f) => Value::Float(Float(f.0)),
        Value::Number(n) => Value::Float(Float(n.0 as f64)),
        Value::String(s) => s.0
            .parse::<f64>()
            .map(|f| Value::Float(Float(f)))
            .unwrap_or(Value::Undefined),
        Value::Boolean(BooleanValue(true)) => Value::Float(Float(1.0)),
        Value::Boolean(BooleanValue(false)) => Value::Float(Float(0.0)),
        Value::Function { .. } => Value::Undefined,
        Value::Array(_) | Value::Dict(_) => Value::Undefined,
        Value::Undefined => Value::Undefined,
    }
}

pub fn std_bool(val: &Value) -> Value {
    match val {
        Value::Boolean(b) => Value::Boolean(BooleanValue(b.0)),
        Value::Number(n) => Value::Boolean(BooleanValue(n.0 != 0)),
        Value::Float(f) => Value::Boolean(BooleanValue(f.0 != 0.0)),
        Value::String(s) => Value::Boolean(BooleanValue(!s.0.is_empty())),
        Value::Array(arr) => Value::Boolean(BooleanValue(!arr.is_empty())),
        Value::Dict(dict) => Value::Boolean(BooleanValue(!dict.is_empty())),
        Value::Undefined => Value::Boolean(BooleanValue(false)),
        Value::Function { .. } => Value::Boolean(BooleanValue(false)),
    }
}
