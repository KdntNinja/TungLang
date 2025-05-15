// Handles Python-like type conversion functions
use crate::value::Value;

pub fn std_int(val: &Value) -> Value {
    match val {
        Value::Number(n) => Value::Number(*n),
        Value::Float(f) => Value::Number(*f as i64),
        Value::String(s) => s
            .parse::<i64>()
            .map(Value::Number)
            .unwrap_or(Value::Undefined),
        Value::Boolean(true) => Value::Number(1),
        Value::Boolean(false) => Value::Number(0),
        &Value::Array(_) | &Value::Dict(_) => Value::Undefined,
        Value::Function { .. } => Value::Undefined,
        _ => Value::Undefined,
    }
}

pub fn std_str(val: &Value) -> Value {
    match val {
        Value::String(s) => Value::String(s.clone()),
        Value::Number(n) => Value::String(n.to_string()),
        Value::Float(f) => Value::String(f.to_string()),
        Value::Boolean(b) => Value::String(b.to_string()),
        Value::Undefined => Value::String("undefined".to_string()),
        Value::Array(arr) => {
            let items: Vec<String> = arr
                .iter()
                .map(|v| match v {
                    Value::String(s) => format!("\"{}\"", s),
                    _ => format!("{}", v),
                })
                .collect();
            Value::String(format!("[{}]", items.join(", ")))
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
            Value::String(format!("{{{}}}", items.join(", ")))
        }
        &Value::Function { .. } => Value::String("<function>".to_string()),
    }
}

// Convert to float (Python-like)
pub fn std_float(val: &Value) -> Value {
    match val {
        Value::Float(f) => Value::Float(*f),
        Value::Number(n) => Value::Float(*n as f64),
        Value::String(s) => s
            .parse::<f64>()
            .map(Value::Float)
            .unwrap_or(Value::Undefined),
        Value::Boolean(true) => Value::Float(1.0),
        Value::Boolean(false) => Value::Float(0.0),
        &Value::Function { .. } => Value::Undefined,
        _ => Value::Undefined,
    }
}

// Convert to boolean (Python-like)
pub fn std_bool(val: &Value) -> Value {
    match val {
        Value::Boolean(b) => Value::Boolean(*b),
        Value::Number(n) => Value::Boolean(*n != 0),
        Value::Float(f) => Value::Boolean(*f != 0.0),
        Value::String(s) => Value::Boolean(!s.is_empty()),
        Value::Array(arr) => Value::Boolean(!arr.is_empty()),
        Value::Dict(dict) => Value::Boolean(!dict.is_empty()),
        Value::Undefined => Value::Boolean(false),
        &Value::Function { .. } => Value::Boolean(false),
    }
}
