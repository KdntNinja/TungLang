// Handles arithmetic and logical operators for TungLang
use crate::value::Value;

/// Applies a binary operator to two Values
pub fn apply_operator(left: Value, right: Value, op: &str) -> Value {
    match (left, right, op) {
        // Arithmetic
        (Value::Integer(left_integer), Value::Integer(right_integer), "+") => Value::Integer(left_integer + right_integer),
        (Value::Integer(left_integer), Value::Integer(right_integer), "-") => Value::Integer(left_integer - right_integer),
        (Value::Integer(left_integer), Value::Integer(right_integer), "*") => Value::Integer(left_integer * right_integer),
        (Value::Integer(left_integer), Value::Integer(right_integer), "/") => Value::Integer(left_integer / right_integer),
        (Value::Integer(left_integer), Value::Integer(right_integer), "%") => Value::Integer(left_integer % right_integer),
        (Value::FloatNumber(left_float), Value::FloatNumber(right_float), "+") => Value::FloatNumber(left_float + right_float),
        (Value::FloatNumber(left_float), Value::FloatNumber(right_float), "-") => Value::FloatNumber(left_float - right_float),
        (Value::FloatNumber(left_float), Value::FloatNumber(right_float), "*") => Value::FloatNumber(left_float * right_float),
        (Value::FloatNumber(left_float), Value::FloatNumber(right_float), "/") => Value::FloatNumber(left_float / right_float),
        // String concatenation
        (Value::String(l), Value::String(r), "+") => Value::String(l + &r),
        // Equality
        (Value::Integer(left_integer), Value::Integer(right_integer), "==") => Value::Boolean(left_integer == right_integer),
        (Value::FloatNumber(left_float), Value::FloatNumber(right_float), "==") => Value::Boolean(left_float == right_float),
        (Value::String(l), Value::String(r), "==") => Value::Boolean(l == r),
        (Value::Boolean(l), Value::Boolean(r), "==") => Value::Boolean(l == r),
        // Inequality
        (Value::Integer(left_integer), Value::Integer(right_integer), "!=") => Value::Boolean(left_integer != right_integer),
        (Value::FloatNumber(left_float), Value::FloatNumber(right_float), "!=") => Value::Boolean(left_float != right_float),
        (Value::String(l), Value::String(r), "!=") => Value::Boolean(l != r),
        (Value::Boolean(l), Value::Boolean(r), "!=") => Value::Boolean(l != r),
        // Comparison
        (Value::Integer(left_integer), Value::Integer(right_integer), op)
            if matches!(op, ">" | "<" | ">=" | "<=") => {
            let result = match op {
                ">" => left_integer > right_integer,
                "<" => left_integer < right_integer,
                ">=" => left_integer >= right_integer,
                "<=" => left_integer <= right_integer,
                _ => unreachable!(),
            };
            Value::Boolean(result)
        }
        (Value::FloatNumber(left_float), Value::FloatNumber(right_float), op)
            if matches!(op, ">" | "<" | ">=" | "<=") => {
            let result = match op {
                ">" => left_float > right_float,
                "<" => left_float < right_float,
                ">=" => left_float >= right_float,
                "<=" => left_float <= right_float,
                _ => unreachable!(),
            };
            Value::Boolean(result)
        }
        (Value::String(l), Value::String(r), op)
            if matches!(op, ">" | "<" | ">=" | "<=") => {
            let res = match op {
                ">" => l > r,
                "<" => l < r,
                ">=" => l >= r,
                "<=" => l <= r,
                _ => unreachable!(),
            };
            Value::Boolean(res)
        }
        // Logical
        (Value::Boolean(l), Value::Boolean(r), "&&") => Value::Boolean(l && r),
        (Value::Boolean(l), Value::Boolean(r), "||") => Value::Boolean(l || r),
        // Unary
        (Value::Boolean(l), Value::Undefined, "!") => Value::Boolean(!l),
        (Value::Integer(integer_value), Value::Undefined, "-") => Value::Integer(-integer_value),
        (Value::FloatNumber(float_value), Value::Undefined, "-") => Value::FloatNumber(-float_value),
        (Value::Array(_), _, _) | (Value::Dict(_), _, _) => Value::Undefined,
        _ => {
            eprintln!("Error: Unsupported operation or type in expression: {}", op);
            Value::Undefined
        }
    }
}
