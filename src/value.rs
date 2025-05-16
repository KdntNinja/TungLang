use std::collections::HashMap;

use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::hash::{Hash, Hasher};
use std::ops::{Add, Div, Mul, Neg, Not, Rem, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Integer(pub i64);

impl Integer {
    pub fn new(value: i64) -> Self {
        Integer(value)
    }
}

impl Add for Integer {
    type Output = Integer;
    fn add(self, rhs: Integer) -> Integer {
        Integer(self.0 + rhs.0)
    }
}
impl Sub for Integer {
    type Output = Integer;
    fn sub(self, rhs: Integer) -> Integer {
        Integer(self.0 - rhs.0)
    }
}
impl Mul for Integer {
    type Output = Integer;
    fn mul(self, rhs: Integer) -> Integer {
        Integer(self.0 * rhs.0)
    }
}
impl Div for Integer {
    type Output = Integer;
    fn div(self, rhs: Integer) -> Integer {
        Integer(self.0 / rhs.0)
    }
}
impl Rem for Integer {
    type Output = Integer;
    fn rem(self, rhs: Integer) -> Integer {
        Integer(self.0 % rhs.0)
    }
}
impl Neg for Integer {
    type Output = Integer;
    fn neg(self) -> Integer {
        Integer(-self.0)
    }
}
impl std::fmt::Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct FloatNumber(pub f64);

impl FloatNumber {
    pub fn new(value: f64) -> Self {
        FloatNumber(value)
    }
}

impl Add for FloatNumber {
    type Output = FloatNumber;
    fn add(self, rhs: FloatNumber) -> FloatNumber {
        FloatNumber(self.0 + rhs.0)
    }
}
impl Sub for FloatNumber {
    type Output = FloatNumber;
    fn sub(self, rhs: FloatNumber) -> FloatNumber {
        FloatNumber(self.0 - rhs.0)
    }
}
impl Mul for FloatNumber {
    type Output = FloatNumber;
    fn mul(self, rhs: FloatNumber) -> FloatNumber {
        FloatNumber(self.0 * rhs.0)
    }
}
impl Div for FloatNumber {
    type Output = FloatNumber;
    fn div(self, rhs: FloatNumber) -> FloatNumber {
        FloatNumber(self.0 / rhs.0)
    }
}
impl Rem for FloatNumber {
    type Output = FloatNumber;
    fn rem(self, rhs: FloatNumber) -> FloatNumber {
        FloatNumber(self.0 % rhs.0)
    }
}
impl Neg for FloatNumber {
    type Output = FloatNumber;
    fn neg(self) -> FloatNumber {
        FloatNumber(-self.0)
    }
}
impl std::fmt::Display for FloatNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Eq for FloatNumber {}
impl Ord for FloatNumber {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}
impl Hash for FloatNumber {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Not ideal for NaN, but works for most cases
        state.write_u64(self.0.to_bits());
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StringValue(pub String);

impl StringValue {
    pub fn new(s: String) -> Self {
        StringValue(s)
    }
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
    pub fn new(b: bool) -> Self {
        BooleanValue(b)
    }
}
impl Not for BooleanValue {
    type Output = BooleanValue;
    fn not(self) -> BooleanValue {
        BooleanValue(!self.0)
    }
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
    Integer(Integer),
    Float(FloatNumber),
    String(StringValue),
    Boolean(BooleanValue),
    Array(Array),
    Dict(Dict),
    Undefined,
    Function {
        parameters: Vec<String>,
        body: String,
        environment: Dict,
    },
}

pub type BuiltinFn = fn(args: &[Value]) -> Value;

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(Integer(n)) => write!(f, "{}", n),
            Value::Float(FloatNumber(n)) => write!(f, "{}", n),
            Value::String(StringValue(s)) => write!(f, "{}", s),
            Value::Boolean(BooleanValue(b)) => write!(f, "{}", b),
            Value::Array(array) => {
                write!(f, "[")?;
                let mut is_first = true;
                for element in array {
                    if !is_first {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", element)?;
                    is_first = false;
                }
                write!(f, "]")
            }
            Value::Dict(dict) => {
                write!(f, "{{")?;
                let mut is_first = true;
                for (key, value) in dict {
                    if !is_first {
                        write!(f, ", ")?;
                    }
                    write!(f, "\"{}\": {}", key, value)?;
                    is_first = false;
                }
                write!(f, "}}")
            }
            Value::Undefined => write!(f, "undefined"),
            Value::Function { .. } => write!(f, "Function"),
        }
    }
}
