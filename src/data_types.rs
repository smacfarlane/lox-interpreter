use anyhow::Result;

use crate::error::EvaluationError;

#[derive(Clone, Debug, PartialEq)]
pub enum Object {
    Nil,
    Boolean(bool),
    String(String),
    Number(f64),
    // Object(Box<Object>)
}

impl std::ops::Add for Object {
    type Output = Result<Object>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(a), Self::Number(b)) => Ok(Self::Number(a + b)),
            (Self::String(a), Self::String(b)) => Ok(Self::String(a + &b)),
            (Self::String(_), _) | (_, Self::String(_)) => {
                Err(EvaluationError::StringConcatination.into())
            }
            (_, _) => Err(EvaluationError::Arithmatic("add".to_string()).into()),
        }
    }
}

impl std::ops::Sub for Object {
    type Output = Result<Object>;
    fn sub(self, rhs: Self) -> Result<Object> {
        match (self, rhs) {
            (Self::Number(a), Self::Number(b)) => Ok(Self::Number(a - b)),
            (_, _) => Err(EvaluationError::Arithmatic("subtract".to_string()).into()),
        }
    }
}

impl std::ops::Mul for Object {
    type Output = Result<Object>;
    fn mul(self, rhs: Self) -> Result<Object> {
        match (self, rhs) {
            (Self::Number(a), Self::Number(b)) => Ok(Self::Number(a * b)),
            (_, _) => Err(EvaluationError::Arithmatic("multiply".to_string()).into()),
        }
    }
}

impl std::ops::Div for Object {
    type Output = Result<Object>;
    fn div(self, rhs: Object) -> Result<Object> {
        match (self, rhs) {
            (Self::Number(a), Self::Number(b)) => Ok(Self::Number(a / b)),
            (_, _) => Err(EvaluationError::Arithmatic("divide".to_string()).into()),
        }
    }
}

impl std::ops::Neg for Object {
    type Output = Result<Object>;

    fn neg(self) -> Result<Object> {
        match self {
            Self::Number(a) => Ok(Self::Number(a.neg())),
            _ => Err(EvaluationError::Negation.into()),
        }
    }
}

impl std::ops::Not for Object {
    type Output = Result<Object>;

    fn not(self) -> Result<Object> {
        match self {
            Self::Boolean(a) => Ok(Self::Boolean(!a)),
            Self::Nil => Ok(Self::Boolean(false)),
            _ => Ok(Self::Boolean(true)),
        }
    }
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{}", n),
            Self::String(s) => write!(f, "{}", s),
            Self::Boolean(b) => write!(f, "{}", b),
            Self::Nil => write!(f, "nil"),
        }
    }
}

impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::String(a), Self::String(b)) => a.partial_cmp(b),
            (Self::Number(a), Self::Number(b)) => a.partial_cmp(b),
            (_, _) => None,
        }
    }
}