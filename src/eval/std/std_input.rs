use crate::value::{FloatNumber, Integer, StringValue, Value};
// TungLang standard input function
use std::io::{self, Write};

/// Prompts the user and returns their input as a Value (Integer, FloatNumber, or String)
pub fn tunglang_input(prompt_value: &Value) -> Value {
    if let Value::String(prompt_str) = prompt_value {
        print!("{}", prompt_str);
        io::stdout().flush().unwrap();
    }
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    let trimmed_input = user_input.trim_end_matches(['\n', '\r']);
    if let Ok(parsed_integer) = trimmed_input.parse::<i64>() {
        Value::Integer(Integer(parsed_integer))
    } else if let Ok(parsed_float) = trimmed_input.parse::<f64>() {
        Value::FloatNumber(FloatNumber(parsed_float))
    } else {
        Value::String(StringValue(trimmed_input.to_string()))
    }
}
