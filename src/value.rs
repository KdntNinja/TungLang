use std::collections::HashMap;


use std::ops::{Add, Sub, Mul, Div, Rem, Neg, Not};
use std::cmp::{PartialEq, Eq, PartialOrd, Ord, Ordering};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Number(pub i64);

impl Number {
    pub fn new(n: i64) -> Self { Number(n) }
}

impl Add for Number {
    type Output = Number;
    fn add(self, rhs: Number) -> Number { Number(self.0 + rhs.0) }
}
impl Sub for Number {
    type Output = Number;
    fn sub(self, rhs: Number) -> Number { Number(self.0 - rhs.0) }
}
impl Mul for Number {
    type Output = Number;
    fn mul(self, rhs: Number) -> Number { Number(self.0 * rhs.0) }
}
impl Div for Number {
    type Output = Number;
    fn div(self, rhs: Number) -> Number { Number(self.0 / rhs.0) }
}
impl Rem for Number {
    type Output = Number;
    fn rem(self, rhs: Number) -> Number { Number(self.0 % rhs.0) }
}
impl Neg for Number {
    type Output = Number;
    fn neg(self) -> Number { Number(-self.0) }
}
impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Float(pub f64);

impl Float {
    pub fn new(f: f64) -> Self { Float(f) }
}

impl Add for Float {
    type Output = Float;
    fn add(self, rhs: Float) -> Float { Float(self.0 + rhs.0) }
}
impl Sub for Float {
    type Output = Float;
    fn sub(self, rhs: Float) -> Float { Float(self.0 - rhs.0) }
}
impl Mul for Float {
    type Output = Float;
    fn mul(self, rhs: Float) -> Float { Float(self.0 * rhs.0) }
}
impl Div for Float {
    type Output = Float;
    fn div(self, rhs: Float) -> Float { Float(self.0 / rhs.0) }
}
impl Rem for Float {
    type Output = Float;
    fn rem(self, rhs: Float) -> Float { Float(self.0 % rhs.0) }
}
impl Neg for Float {
    type Output = Float;
    fn neg(self) -> Float { Float(-self.0) }
}
impl std::fmt::Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Eq for Float {}
impl Ord for Float {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}
impl Hash for Float {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Not ideal for NaN, but works for most cases
        state.write_u64(self.0.to_bits());
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StringValue(pub String);

impl StringValue {
    pub fn new(s: String) -> Self { StringValue(s) }
    pub fn repeat(&self, n: usize) -> StringValue {
        StringValue(self.0.repeat(n))
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
}
impl std::ops::Add<&StringValue> for StringValue {
    type Output = StringValue;
    fn add(self, rhs: &StringValue) -> StringValue {
        StringValue(self.0 + &rhs.0)
    }
}
impl std::fmt::Display for StringValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BooleanValue(pub bool);

impl BooleanValue {
    pub fn new(b: bool) -> Self { BooleanValue(b) }
}
impl Not for BooleanValue {
    type Output = BooleanValue;
    fn not(self) -> BooleanValue { BooleanValue(!self.0) }
}
impl std::fmt::Display for BooleanValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

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
