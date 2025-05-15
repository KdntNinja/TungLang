#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Array(Vec<Value>),
    Dict(std::collections::HashMap<String, Value>),
    Undefined, // Added to represent undefined values
    Function {
        params: Vec<String>,
        body: String, // Store the function body as a String
        env: std::collections::HashMap<String, Value>,
    },
}

pub type BuiltinFn = fn(&[Value]) -> Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Number(pub i64);

#[derive(Debug, Clone, PartialEq)]
pub struct StringValue(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct BooleanValue(pub bool);

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Float(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
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
            Value::Function {
                params: _,
                body: _,
                env: _,
            } => write!(f, "Function"),
        }
    }
}
