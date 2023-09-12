use crate::ast::{Expr, ExpressionVisitor, StatementVisitor, Stmt};
use crate::data_types::{Clock, Function, Object, Return};
use crate::environment::Environment;
use crate::error::{EvaluationError, RuntimeError};
use crate::token::{Token, TokenType};

use anyhow::{anyhow, Result};
use tracing::instrument;

#[derive(Debug)]
pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut globals = Environment::new();
        globals.define(
            "clock".to_string(),
            Object::Function(std::rc::Rc::new(Clock)),
        );
        Interpreter {
            environment: globals,
        }
    }

    // #[instrument(skip(self), ret, level = "trace")]
    // pub fn resolve(&mut self, name: Expr, depth: usize) {
    // dbg!(&name);
    // println!("Resolve: {}: {}", &name, self.locals.len());
    // self.locals.insert(name.clone(), depth);
    // println!("Resolve: {}: {}", &name, self.locals.len());
    // }

    // pub fn look_up_variable(&self, name: Token) -> Result<Object> {
    //     let lexeme = name.lexeme.clone().unwrap();
    //     let name = Expr::Variable(name);
    //
    //
    //     match self.locals.get(&name) {
    //         Some(distance) => self.environment.get_at(*distance, &lexeme),
    //         None => self.environment.get_global(&lexeme),
    //     }
    // }

    #[instrument(skip(self), ret, level = "trace")]
    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<()> {
        for statement in statements {
            execute(self, &statement)?;
        }

        Ok(())
    }

    #[instrument(skip(self), ret, level = "trace")]
    pub fn with_environment(&self, e: Environment) -> Interpreter {
        Interpreter { environment: e }
    }

    #[instrument(skip(self), ret, level = "trace")]
    pub fn execute_block(&mut self, statements: &Vec<Box<Stmt>>) -> Result<Return> {
        for statement in statements {
            let result = match execute(self, &*statement) {
                Err(e) => Some(Err(e)),
                Ok(Return::Value(r)) => Some(Ok(Return::Value(r))),
                Ok(Return::Bare) => Some(Ok(Return::Bare)),
                Ok(Return::None) => None,
            };

            if let Some(result) = result {
                return result;
            }
        }

        Ok(Return::None)
    }
}

impl StatementVisitor for &mut Interpreter {
    fn visit_block(&mut self, stmts: &Vec<Box<Stmt>>) -> Result<Return> {
        (**self).visit_block(stmts)
    }

    fn visit_print(&mut self, expr: &Expr) -> Result<Return> {
        (**self).visit_print(expr)
    }

    fn visit_expression(&mut self, expr: &Expr) -> Result<Return> {
        (**self).visit_expression(expr)
    }

    fn visit_variable(&mut self, name: &Token, initializer: Option<&Expr>) -> Result<Return> {
        // Disambiguate between StatementVisitor and ExpressionVisitor
        StatementVisitor::visit_variable(*self, name, initializer)
    }
    fn visit_if(&mut self, condition: &Expr, then: &Stmt, els: Option<&Stmt>) -> Result<Return> {
        (**self).visit_if(condition, then, els)
    }
    fn visit_while(&mut self, condition: &Expr, body: &Stmt) -> Result<Return> {
        (**self).visit_while(condition, body)
    }

    fn visit_function(
        &mut self,
        name: &Token,
        parameters: &Vec<Token>,
        body: &Vec<Box<Stmt>>,
    ) -> Result<Return> {
        (**self).visit_function(name, parameters, body)
    }

    fn visit_return(&mut self, token: &Token, expr: Option<&Expr>) -> Result<Return> {
        (**self).visit_return(token, expr)
    }
}

impl StatementVisitor for Interpreter {
    #[instrument(skip(self), ret, level = "trace")]
    fn visit_block(&mut self, stmts: &Vec<Box<Stmt>>) -> Result<Return> {
        self.environment.new_scope();
        let result = self.execute_block(stmts);
        self.environment.end_scope();
        result
    }

    #[instrument(skip(self), ret, level = "trace")]
    fn visit_print(&mut self, expr: &Expr) -> Result<Return> {
        let value = evaluate(self, expr)?;
        println!("{}", value);
        Ok(Return::None)
    }

    #[instrument(skip(self), ret, level = "trace")]
    fn visit_expression(&mut self, expr: &Expr) -> Result<Return> {
        evaluate(self, expr)?;
        Ok(Return::None)
    }

    #[instrument(skip(self), ret, level = "trace")]
    fn visit_variable(&mut self, name: &Token, initializer: Option<&Expr>) -> Result<Return> {
        let mut value = Object::Nil;

        if let Some(initializer) = initializer {
            value = evaluate(self, initializer)?;
        }
        let name = name
            .lexeme
            .clone()
            .ok_or(RuntimeError::UnexpectedToken(name.clone()))?;

        self.environment.define(name, value);

        Ok(Return::None)
    }

    #[instrument(skip(self), ret, level = "trace")]
    fn visit_if(&mut self, condition: &Expr, then: &Stmt, els: Option<&Stmt>) -> Result<Return> {
        if evaluate(self, condition)?.is_truthy() {
            execute(self, then)
        } else if let Some(els) = els {
            execute(self, els)
        } else {
            Ok(Return::None)
        }
    }

    #[instrument(skip(self), ret, level = "trace")]
    fn visit_while(&mut self, condition: &Expr, body: &Stmt) -> Result<Return> {
        while evaluate(self, condition)?.is_truthy() {
            let ret = execute(self, body)?;
            if ret.is_explicit() {
                return Ok(ret);
            }
        }

        Ok(Return::None)
    }

    #[instrument(skip(self), ret, level = "trace")]
    fn visit_function(
        &mut self,
        name: &Token,
        arguments: &Vec<Token>,
        body: &Vec<Box<Stmt>>,
    ) -> Result<Return> {
        let function = Function::new(
            name.clone(),
            arguments.clone(),
            body.clone(),
            self.environment.clone(),
        );
        let name = name.lexeme.clone().unwrap();

        self.environment
            .define(name, Object::Function(std::rc::Rc::new(function)));

        Ok(Return::None)
    }

    #[instrument(skip(self), ret, level = "trace")]
    fn visit_return(&mut self, _token: &Token, expr: Option<&Expr>) -> Result<Return> {
        match expr {
            Some(e) => Ok(Return::Value(evaluate(self, &e)?)),
            None => Ok(Return::Bare),
        }
    }
}

impl ExpressionVisitor<Object> for Interpreter {
    #[instrument(skip(self), ret, level = "trace")]
    fn visit_assignment(&mut self, name: &Token, expr: &Expr) -> Result<Object> {
        let value = evaluate(self, expr)?;
        let lexeme = name
            .lexeme
            .clone()
            .ok_or(RuntimeError::UnexpectedToken(name.clone()))?;

        self.environment.assign(lexeme, value.clone())?;
        // let name = Expr::Assign {
        //     name: name.clone(),
        //     value: Box::new(expr.clone()),
        // };
        // match self.locals.get(&name) {
        //     Some(distance) => self
        //         .environment
        //         .assign_at(*distance, lexeme, value.clone())?,
        //     None => self.environment.assign_global(lexeme, value.clone())?,
        // }

        Ok(value)
    }

    #[instrument(skip(self), ret, level = "trace")]
    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<Object> {
        let left: Object = evaluate(self, left)?.try_into()?;
        let right: Object = evaluate(self, right)?.try_into()?;

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

    #[instrument(skip(self), ret, level = "trace")]
    fn visit_call(
        &mut self,
        callee: &Expr,
        _paren: &Token,
        arguments: &Vec<Box<Expr>>,
    ) -> Result<Object> {
        let callee = evaluate(self, callee)?;

        let arguments = arguments
            .iter()
            .map(|arg| evaluate(self, arg))
            .collect::<Result<Vec<Object>>>()?;

        let callee = match callee {
            Object::Function(f) => Ok(f),
            _ => Err(anyhow!("attempting to call primitive as function")),
        }?;

        match callee.call(self, &arguments)? {
            Return::Value(e) => Ok(e),
            Return::Bare => Ok(Object::Nil),
            Return::None => Ok(Object::Nil), // TODO: What is the right thing to do here?
        }
    }

    #[instrument(skip(self), ret, level = "trace")]
    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> Result<Object> {
        let right = evaluate(self, right)?;

        match operator.token_type {
            TokenType::Minus => -right,
            TokenType::Bang => Ok(!right),
            _ => Err(anyhow!("invalid operation")),
        }
    }

    #[instrument(skip(self), ret, level = "trace")]
    fn visit_grouping(&mut self, grouping: &Expr) -> Result<Object> {
        evaluate(self, grouping)
    }

    #[instrument(skip(self), ret, level = "trace")]
    fn visit_literal(&mut self, literal: &Object) -> Result<Object> {
        Ok(literal.clone())
    }

    #[instrument(skip(self), ret, level = "trace")]
    fn visit_logical(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<Object> {
        let left = evaluate(self, left)?;

        match (operator.token_type.clone(), left.is_truthy()) {
            (TokenType::Or, false) => evaluate(self, right),
            (TokenType::Or, true) | (_, false) => Ok(left),
            (_, _) => evaluate(self, right),
        }
    }

    #[instrument(skip(self), ret, level = "trace")]
    fn visit_variable(&mut self, token: &Token) -> Result<Object> {
        self.environment.get(&token.lexeme.clone().unwrap())
        // self.look_up_variable(token.clone())
    }
}

fn evaluate<V, T>(visitor: &mut V, expression: &Expr) -> Result<T>
where
    V: ExpressionVisitor<T>,
{
    expression.accept(visitor)
}

// fn execute
fn execute<V>(visitor: &mut V, statement: &Stmt) -> Result<Return>
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
