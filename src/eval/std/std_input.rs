// Handles the TungLang input() built-in function
use crate::value::{Value, StringValue};
use std::io::{self, Write};

/// Prompts the user and returns their input as a Value (Number, Float, or StringValue)
pub fn std_input(prompt: &Value) -> Value {
    if let Value::StringValue(prompt) = prompt {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
    }
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim_end_matches(['\n', '\r']);
    if let Ok(n) = input.parse::<i64>() {
        Value::Number(n)
    } else if let Ok(f) = input.parse::<f64>() {
        Value::Float(f)
    } else {
        Value::StringValue(input.to_string())
    }
}
