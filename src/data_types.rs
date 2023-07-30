use anyhow::Result;

use std::rc::Rc;

use crate::ast::Stmt;
use crate::environment::Environment;
use crate::error::EvaluationError;
use crate::interpreter::Interpreter;
use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Object {
    Nil,
    Boolean(bool),
    String(String),
    Number(f64),
    Function(Rc<dyn Callable>),
    // Function(C), // Object(Box<Object>)
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Nil, Self::Nil) => true,
            (Self::Boolean(ref l), Self::Boolean(ref r)) => l == r,
            (Self::String(ref l), Self::String(ref r)) => l == r,
            (Self::Number(ref l), Self::Number(ref r)) => l == r,
            (_, _) => false,
        }
    }
}

impl Object {
    pub fn is_truthy(&self) -> bool {
        match !!&*self {
            Self::Boolean(true) => true,
            Self::Boolean(false) => false,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum Return {
    Value(Object), // return "foo";
    Bare,          // return;
    None,          // <no return statement>
}

impl Return {
    pub fn is_explicit(&self) -> bool {
        match self {
            Self::Value(_) | Self::Bare => true,
            Self::None => false,
        }
    }
}

pub trait Callable: std::fmt::Debug {
    fn arity(&self) -> u8; // Max 255 arguments
    fn call(&self, i: &Interpreter, arguments: &Vec<Object>) -> Result<Return>;
}

#[derive(Debug, PartialEq)]
pub struct Function {
    name: Token, // Identifier
    params: Vec<Token>,
    body: Vec<Box<Stmt>>, // Block
    closure: Environment,
}

impl Function {
    pub fn new(
        name: Token,
        params: Vec<Token>,
        body: Vec<Box<Stmt>>,
        closure: Environment,
    ) -> Function {
        Function {
            name,
            params,
            body,
            closure,
        }
    }
}

impl Callable for Function {
    fn arity(&self) -> u8 {
        self.params.len() as u8
    }
    fn call(&self, interpreter: &Interpreter, arguments: &Vec<Object>) -> Result<Return> {
        let mut environment = Environment::contains(&self.closure);

        for (param, arg) in self.params.iter().zip(arguments.iter()) {
            let param = param.lexeme.clone().unwrap(); // TODO: Danger zone
            environment.define(param, arg.clone());
        }

        let mut interpreter = interpreter.with_environment(environment);

        interpreter.execute_block(&self.body)
    }
}

#[derive(Clone, Debug)]
pub struct Clock;

impl Callable for Clock {
    fn arity(&self) -> u8 {
        0
    }
    fn call(&self, _: &Interpreter, _: &Vec<Object>) -> Result<Return> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .expect("SystemTime before 1970-01-01 00:00:00 UTC");
        let now = now.as_secs_f64() * 1000.0;

        Ok(Return::Value(Object::Number(now)))
    }
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
            // (Self::Number(a), Self::Nil) | (Self::Nil, Self::Number(a)) => Ok(Self::Number(a)), // nil -> 0 in Lox
            (_, _) => Err(EvaluationError::Arithmatic("add".to_string()).into()),
        }
    }
}

impl std::ops::Sub for Object {
    type Output = Result<Object>;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(a), Self::Number(b)) => Ok(Self::Number(a - b)),
            (_, _) => Err(EvaluationError::Arithmatic("subtract".to_string()).into()),
        }
    }
}

impl std::ops::Mul for Object {
    type Output = Result<Object>;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(a), Self::Number(b)) => Ok(Self::Number(a * b)),
            (_, _) => Err(EvaluationError::Arithmatic("multiply".to_string()).into()),
        }
    }
}

impl std::ops::Div for Object {
    type Output = Result<Object>;
    fn div(self, rhs: Object) -> Self::Output {
        match (self, rhs) {
            (Self::Number(a), Self::Number(b)) => Ok(Self::Number(a / b)),
            (_, _) => Err(EvaluationError::Arithmatic("divide".to_string()).into()),
        }
    }
}

impl std::ops::Neg for Object {
    type Output = Result<Object>;

    fn neg(self) -> Self::Output {
        match self {
            Self::Number(a) => Ok(Self::Number(a.neg())),
            _ => Err(EvaluationError::Negation.into()),
        }
    }
}

impl std::ops::Not for Object {
    type Output = Object;

    fn not(self) -> Self::Output {
        match self {
            Self::Boolean(a) => Self::Output::Boolean(!a),
            Self::Nil => Self::Output::Boolean(false),
            _ => Self::Output::Boolean(true),
        }
    }
}

impl std::ops::Not for &Object {
    type Output = Object;
    fn not(self) -> Self::Output {
        match *self {
            Object::Boolean(a) => Self::Output::Boolean(!a),
            Object::Nil => Self::Output::Boolean(false),
            _ => Self::Output::Boolean(true),
        }
    }
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Number(ref n) => write!(f, "{}", n),
            Self::String(ref s) => write!(f, "{}", s),
            Self::Boolean(ref b) => write!(f, "{}", b),
            Self::Nil => write!(f, "nil"),
            Self::Function(_) => write!(f, "<fn placehodler>"),
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
