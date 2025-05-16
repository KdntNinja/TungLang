// Handles the TungLang input() built-in function
use crate::value::{Value, StringValue};
use std::io::{self, Write};

/// Prompts the user and returns their input as a Value (Number, Float, or StringValue)
pub fn std_input(prompt: &Value) -> Value {
    if let Value::String(prompt) = prompt {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
    }
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim_end_matches(['\n', '\r']);
    if let Ok(n) = input.parse::<i64>() {
        Value::Number(Number(n))
    } else if let Ok(f) = input.parse::<f64>() {
        Value::Float(Float(f))
    } else {
        Value::String(StringValue(input.to_string()))
    }
}
