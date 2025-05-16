use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Number(pub i64);

#[derive(Debug, Clone, PartialEq)]
pub struct Float(pub f64);

#[derive(Debug, Clone, PartialEq)]
pub struct StringValue(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct BooleanValue(pub bool);

pub type Array = Vec<Value>;
pub type Dict = HashMap<String, Value>;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(Number),
    Float(Float),
    String(StringValue),
    Boolean(BooleanValue),
    Array(Array),
    Dict(Dict),
    Undefined,
    Function {
        params: Vec<String>,
        body: String,
        env: Dict,
    },
}

pub type BuiltinFn = fn(args: &[Value]) -> Value;

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(Number(n)) => write!(f, "{}", n),
            Value::Float(Float(n)) => write!(f, "{}", n),
            Value::String(StringValue(s)) => write!(f, "{}", s),
            Value::Boolean(BooleanValue(b)) => write!(f, "{}", b),
            Value::Array(a) => {
                write!(f, "[")?;
                let mut first = true;
                for item in a {
                    if !first {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                    first = false;
                }
                write!(f, "]")
            }
            Value::Dict(d) => {
                write!(f, "{{")?;
                let mut first = true;
                for (key, val) in d {
                    if !first {
                        write!(f, ", ")?;
                    }
                    write!(f, "\"{}\": {}", key, val)?;
                    first = false;
                }
                write!(f, "}}")
            }
            Value::Undefined => write!(f, "undefined"),
            Value::Function { .. } => write!(f, "Function"),
        }
    }
}
