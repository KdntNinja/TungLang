// Handles the TungLang print() built-in function
use crate::value::{Value, Number, Float, StringValue, BooleanValue};

/// Prints a Value to stdout
pub fn std_print(val: &Value) {
    match val {
        Value::String(s) => println!("{}", s),
        Value::Number(n) => println!("{}", n),
        Value::Float(f) => println!("{}", f),
        Value::Boolean(b) => println!("{}", b),
        Value::Array(arr) => println!("{:?}", arr),
        Value::Dict(map) => println!("{:?}", map),
        Value::Undefined => println!("undefined"),
        Value::Function { .. } => println!("<function>"),
    }
}
