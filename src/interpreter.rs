use crate::data_types::Object;
use crate::error::EvaluationError;
use crate::expr::{Expr, Visitor};
use crate::token::{Token, TokenType};

use anyhow::{anyhow, Result};

#[derive(Clone, Copy)]
struct Interpreter();

impl Visitor<Object> for Interpreter {
    fn visit_binary(&self, left: &Expr, operator: &Token, right: &Expr) -> Result<Object> {
        let left = evaluate(self, left)?;
        let right = evaluate(self, right)?;

        match operator.token_type {
            TokenType::Minus => left - right,
            TokenType::Slash => left / right,
            TokenType::Star => left * right,
            TokenType::Plus => left + right,
            TokenType::Greater => match left.partial_cmp(&right) {
                Some(b) => Ok(Object::Boolean(b.is_gt())),
                None => Err(EvaluationError::Comparision(">".to_string()).into()),
            },
            TokenType::GreaterEqual => match left.partial_cmp(&right) {
                Some(b) => Ok(Object::Boolean(b.is_ge())),
                None => Err(EvaluationError::Comparision(">=".to_string()).into()),
            },
            TokenType::Less => match left.partial_cmp(&right) {
                Some(b) => Ok(Object::Boolean(b.is_lt())),
                None => Err(EvaluationError::Comparision("<".to_string()).into()),
            },
            TokenType::LessEqual => match left.partial_cmp(&right) {
                Some(b) => Ok(Object::Boolean(b.is_le())),
                None => Err(EvaluationError::Comparision("<=".to_string()).into()),
            },
            TokenType::EqualEqual => Ok(Object::Boolean(left.eq(&right))),
            TokenType::BangEqual => Ok(Object::Boolean(!left.eq(&right))),
            _ => Err(anyhow!("invalid operation")),
        }
    }
    fn visit_unary(&self, operator: &Token, right: &Expr) -> Result<Object> {
        let right = evaluate(self, right)?;

        match operator.token_type {
            TokenType::Minus => -right,
            TokenType::Bang => !right,
            _ => Err(anyhow!("invalid operation")),
        }
    }
    fn visit_grouping(&self, grouping: &Expr) -> Result<Object> {
        evaluate(self, grouping)
    }
    fn visit_literal(&self, literal: &Object) -> Result<Object> {
        Ok(literal.clone())
    }
}

pub(crate) fn interpret(expression: &Expr) -> Result<String> {
    let interpreter = Interpreter();
    let value = evaluate(&interpreter, expression)?;

    // Hack to add tests for now
    Ok(format!("{}", value))
}

fn evaluate<V, T>(visitor: &V, expression: &Expr) -> Result<T>
where
    V: Visitor<T> + Copy,
{
    expression.accept(*visitor)
}

#[cfg(test)]
mod test {
    #[test]
    #[allow(unused_must_use)]
    fn interpreter() {
        macro_rules! interpret {
            ($input:expr) => {{
                let mut scanner = crate::scanner::Scanner::new($input.to_string());
                let tokens = scanner.scan_tokens().unwrap();

                let mut parser = crate::parser::Parser::new(tokens);
                let expr = parser.parse().unwrap();

                crate::interpreter::interpret(&Box::new(expr))
            }};
        }

        assert_eq!("42", interpret!("21 + 21").unwrap());
        // assert_eq!("12.3123", interpret!("150 - 137.6877").unwrap()); // TODO: Floating point comparisons
        assert_eq!("true", interpret!("!false").unwrap());
        assert_eq!("true", interpret!("true == !false").unwrap());
        assert_eq!("false", interpret!("10 > 25").unwrap());
        assert_eq!("true", interpret!("25.0 == 25").unwrap());
        assert_eq!("25", interpret!("12.5 * 2").unwrap());
        // assert_eq!("33.458", interpret!("543.20 / 16.235").unwrap()); // TODO: Floating point comparisons
        assert_eq!("false", interpret!("true == 25").unwrap());
        assert_eq!("-125", interpret!("-125").unwrap());
        assert_eq!("foobar", interpret!("\"foo\" + \"bar\"").unwrap());
        assert!(interpret!("true + 3").is_err());
    }
}
