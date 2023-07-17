use crate::ast::{Expr, ExpressionVisitor, StatementVisitor, Stmt};
use crate::data_types::Object;
use crate::environment::{self, Environment};
use crate::error::{EvaluationError, RuntimeError};
use crate::token::{Token, TokenType};

use anyhow::{anyhow, Result};

// #[derive(Clone)]
pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new(),
        }
    }
    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<()> {
        for statement in statements {
            execute(self, &statement)?;
        }

        Ok(())
    }

    fn execute_block(&mut self, statements: &Vec<Box<Stmt>>, e: Environment) -> Result<()> {
        let previous = self.environment.clone();
        let mut had_error = Ok(());

        self.environment = e.clone();
        for statement in statements {
            if let Err(e) = execute(self, &*statement) {
                had_error = Err(e);
                break;
            }
        }

        self.environment = previous;
        had_error
    }
}

impl StatementVisitor for &mut Interpreter {
    fn visit_block(&mut self, stmts: &Vec<Box<Stmt>>) -> Result<()> {
        (**self).visit_block(stmts)
    }

    fn visit_print(&mut self, expr: &Expr) -> Result<()> {
        (**self).visit_print(expr)
    }
    fn visit_expression(&mut self, expr: &Expr) -> Result<()> {
        (**self).visit_expression(expr)
    }
    fn visit_variable(&mut self, name: &Token, initializer: Option<&Expr>) -> Result<()> {
        // Disambiguate between StatementVisitor and ExpressionVisitor
        StatementVisitor::visit_variable(*self, name, initializer)
    }
    fn visit_if(&mut self, condition: &Expr, then: &Stmt, els: Option<&Stmt>) -> Result<()> {
        (**self).visit_if(condition, then, els)
    }
}

impl StatementVisitor for Interpreter {
    fn visit_block(&mut self, stmts: &Vec<Box<Stmt>>) -> Result<()> {
        self.execute_block(stmts, Environment::enclose(&self.environment))
    }

    fn visit_print(&mut self, expr: &Expr) -> Result<()> {
        let value = evaluate(self, expr)?;

        println!("{}", value);
        Ok(())
    }
    fn visit_expression(&mut self, expr: &Expr) -> Result<()> {
        evaluate(self, expr)?;
        Ok(())
    }
    fn visit_variable(&mut self, name: &Token, initializer: Option<&Expr>) -> Result<()> {
        let mut value = Object::Nil;

        if let Some(initializer) = initializer {
            value = evaluate(self, initializer)?;
        }
        let name = name
            .lexeme
            .clone()
            .ok_or(RuntimeError::UnexpectedToken(name.clone()))?;
        self.environment.define(name, value);

        Ok(())
    }

    fn visit_if(&mut self, condition: &Expr, then: &Stmt, els: Option<&Stmt>) -> Result<()> {
        if evaluate(self, condition)?.is_truthy() {
            execute(self, then)?;
        } else if let Some(els) = els {
            execute(self, els)?;
        }

        Ok(())
    }
}

impl ExpressionVisitor<Object> for Interpreter {
    fn visit_assignment(&mut self, name: &Token, value: &Expr) -> Result<Object> {
        let value = evaluate(self, value)?;
        let name = name
            .lexeme
            .clone()
            .ok_or(RuntimeError::UnexpectedToken(name.clone()))?;
        self.environment.assign(name, value.clone())?;

        Ok(value)
    }
    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<Object> {
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
    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> Result<Object> {
        let right = evaluate(self, right)?;

        match operator.token_type {
            TokenType::Minus => -right,
            TokenType::Bang => Ok(!right),
            _ => Err(anyhow!("invalid operation")),
        }
    }
    fn visit_grouping(&mut self, grouping: &Expr) -> Result<Object> {
        evaluate(self, grouping)
    }
    fn visit_literal(&mut self, literal: &Object) -> Result<Object> {
        Ok(literal.clone())
    }
    fn visit_variable(&mut self, token: &Token) -> Result<Object> {
        self.environment.get(token)
    }
}

fn evaluate<V, T>(visitor: &mut V, expression: &Expr) -> Result<T>
where
    V: ExpressionVisitor<T>,
{
    expression.accept(visitor)
}

// fn execute
fn execute<V>(visitor: &mut V, statement: &Stmt) -> Result<()>
where
    V: StatementVisitor,
{
    statement.accept(visitor)
}

#[cfg(test)]
mod test {
    // #[test]
    // fn interpreter() {
    //     macro_rules! interpret {
    //         ($input:expr) => {{
    //             let mut scanner = crate::scanner::Scanner::new($input.to_string());
    //             let tokens = scanner.scan_tokens().unwrap();

    //             let mut parser = crate::parser::Parser::new(tokens);
    //             let expr = parser.parse().unwrap();

    //             crate::interpreter::interpret(&Box::new(expr))
    //         }};
    //     }

    //     assert_eq!("42", interpret!("21 + 21").unwrap());
    //     // assert_eq!("12.3123", interpret!("150 - 137.6877").unwrap()); // TODO: Floating point comparisons
    //     assert_eq!("true", interpret!("!false").unwrap());
    //     assert_eq!("true", interpret!("true == !false").unwrap());
    //     assert_eq!("false", interpret!("10 > 25").unwrap());
    //     assert_eq!("true", interpret!("25.0 == 25").unwrap());
    //     assert_eq!("25", interpret!("12.5 * 2").unwrap());
    //     // assert_eq!("33.458", interpret!("543.20 / 16.235").unwrap()); // TODO: Floating point comparisons
    //     assert_eq!("false", interpret!("true == 25").unwrap());
    //     assert_eq!("-125", interpret!("-125").unwrap());
    //     assert_eq!("foobar", interpret!("\"foo\" + \"bar\"").unwrap());
    //     assert!(interpret!("true + 3").is_err());
    // }
}
