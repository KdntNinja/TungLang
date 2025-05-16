// file: /home/kaiden/RustroverProjects/TungLang/src/eval/std_math.rs
// Python-like math functions for TungLang
use crate::value::{Value, Number, Float, StringValue, BooleanValue};

// min function
pub fn std_min(args: &[Value]) -> Value {
    if args.is_empty() {
        return Value::Undefined;
    }

    match &args[0] {
        Value::Array(array) => {
            if array.is_empty() {
                return Value::Undefined;
            }

            let mut min_value = array[0].clone();

            for value in array {
                match (value, &min_value) {
                    (Value::Number(Number(n1)), Value::Number(Number(n2))) if n1 < n2 => min_value = value.clone(),
                    (Value::Float(Float(f1)), Value::Float(Float(f2))) if f1 < f2 => min_value = value.clone(),
                    (Value::Number(Number(n)), Value::Float(Float(f))) if (*n as f64) < *f => {
                        min_value = value.clone()
                    }
                    (Value::Float(Float(f)), Value::Number(Number(n))) if *f < (*n as f64) => {
                        min_value = value.clone()
                    }
                    (Value::StringValue(StringValue(s1)), Value::StringValue(StringValue(s2))) if s1 < s2 => min_value = value.clone(),
                    _ => {}
                }
            }

            min_value
        }
        _ => {
            // Find min among the arguments
            let mut min_value = args[0].clone();

            for value in args {
                match (value, &min_value) {
                    (Value::Number(Number(n1)), Value::Number(Number(n2))) if n1 < n2 => min_value = value.clone(),
                    (Value::Float(Float(f1)), Value::Float(Float(f2))) if f1 < f2 => min_value = value.clone(),
                    (Value::Number(Number(n)), Value::Float(Float(f))) if (*n as f64) < *f => {
                        min_value = value.clone()
                    }
                    (Value::Float(Float(f)), Value::Number(Number(n))) if *f < (*n as f64) => {
                        min_value = value.clone()
                    }
                    (Value::StringValue(StringValue(s1)), Value::StringValue(StringValue(s2))) if s1 < s2 => min_value = value.clone(),
                    _ => {}
                }
            }

            min_value
        }
    }
}

// max function
pub fn std_max(args: &[Value]) -> Value {
    if args.is_empty() {
        return Value::Undefined;
    }

    match &args[0] {
        Value::Array(array) => {
            if array.is_empty() {
                return Value::Undefined;
            }

            let mut max_value = array[0].clone();

            for value in array {
                match (value, &max_value) {
                    (Value::Number(Number(n1)), Value::Number(Number(n2))) if n1 > n2 => max_value = value.clone(),
                    (Value::Float(Float(f1)), Value::Float(Float(f2))) if f1 > f2 => max_value = value.clone(),
                    (Value::Number(Number(n)), Value::Float(Float(f))) if (*n as f64) > *f => {
                        max_value = value.clone()
                    }
                    (Value::Float(Float(f)), Value::Number(Number(n))) if *f > (*n as f64) => {
                        max_value = value.clone()
                    }
                    (Value::StringValue(StringValue(s1)), Value::StringValue(StringValue(s2))) if s1 > s2 => max_value = value.clone(),
                    _ => {}
                }
            }

            max_value
        }
        _ => {
            // Find max among the arguments
            let mut max_value = args[0].clone();

            for value in args {
                match (value, &max_value) {
                    (Value::Number(Number(n1)), Value::Number(Number(n2))) if n1 > n2 => max_value = value.clone(),
                    (Value::Float(Float(f1)), Value::Float(Float(f2))) if f1 > f2 => max_value = value.clone(),
                    (Value::Number(Number(n)), Value::Float(Float(f))) if (*n as f64) > *f => {
                        max_value = value.clone()
                    }
                    (Value::Float(Float(f)), Value::Number(Number(n))) if *f > (*n as f64) => {
                        max_value = value.clone()
                    }
                    (Value::StringValue(StringValue(s1)), Value::StringValue(StringValue(s2))) if s1 > s2 => max_value = value.clone(),
                    _ => {}
                }
            }

            max_value
        }
    }
}

// sum function
pub fn std_sum(args: &[Value]) -> Value {
    if args.is_empty() {
        return Value::Number(Number(0));
    }

    match &args[0] {
        Value::Array(array) => {
            if array.is_empty() {
                return Value::Number(Number(0));
            }

            let mut sum_int = 0i64;
            let mut sum_float = 0.0f64;
            let mut is_float = false;

            for value in array {
                match value {
                    Value::Number(Number(n)) => {
                        if is_float {
                            sum_float += *n as f64;
                        } else {
                            sum_int += n;
                        }
                    }
                    Value::Float(Float(f)) => {
                        if !is_float {
                            sum_float = sum_int as f64;
                            is_float = true;
                        }
                        sum_float += f;
                    }
                    _ => return Value::Undefined, // Non-numeric value
                }
            }

            if is_float {
                Value::Float(Float(sum_float))
            } else {
                Value::Number(Number(sum_int))
            }
        }
        _ => Value::Undefined,
    }
}

// round function
pub fn std_round(args: &[Value]) -> Value {
    if args.is_empty() {
        return Value::Undefined;
    }

    let value = &args[0];
    let digits = if args.len() > 1 {
        match &args[1] {
            Value::Number(Number(n)) => *n,
            _ => 0,
        }
    } else {
        0
    };

    match value {
        Value::Number(Number(n)) => Value::Number(Number(*n)),
        Value::Float(Float(f)) => {
            if digits == 0 {
                Value::Number(Number(f.round() as i64))
            } else {
                let factor = 10.0f64.powi(digits as i32);
                Value::Float(Float((f * factor).round() / factor))
            }
        }
        _ => Value::Undefined,
    }
}
