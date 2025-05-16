// Handles the TungLang range() built-in function
use crate::value::{Integer, Value};

/// Returns an array of numbers from start to end-1
pub fn std_range(args: &[Value]) -> Value {
    let (start, end): (i64, i64) = match (args.get(0), args.get(1)) {
        (Some(Value::Integer(Integer(s))), Some(Value::Integer(Integer(e)))) => (*s, *e),
        (Some(Value::Integer(Integer(s))), None) => (0, *s),
        _ => (0, 0),
    };
    Value::Array((start..end).map(|n| Value::Integer(Integer(n))).collect())
}
