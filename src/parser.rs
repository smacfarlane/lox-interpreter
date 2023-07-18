use crate::{
    ast::{Expr, Stmt},
    data_types::Object,
    error::ParseError,
    token::{Token, TokenType, TokenTypeDiscriminants},
};

use anyhow::{anyhow, Result};

#[derive(Debug)]
pub(crate) struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>> {
        let mut statements = Vec::new();

        while self.peek().is_some() {
            // TODO: Capture the error
            if let Some(d) = self.declaration()? {
                statements.push(d);
            }
        }

        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Option<Stmt>> {
        let token = self.peek().ok_or(anyhow!("expected token"))?;

        let statement = match token.token_type {
            TokenType::Var => {
                self.next();
                self.var_declaration()
            }
            _ => self.statement(),
        };

        let statement = match statement {
            Ok(statement) => Some(statement),
            Err(_) => {
                self.synchronize();
                None
            }
        };

        Ok(statement)
    }

    fn var_declaration(&mut self) -> Result<Stmt> {
        let name = {
            let token = self.peek().ok_or(anyhow!("expected token"))?;
            match token.token_type {
                TokenType::Identifier(_) => self.next().ok_or(anyhow!("expected_token")),
                _ => Err(anyhow!("expected identifier")),
            }
        }?;

        let token = self.peek().ok_or(anyhow!("expected token"))?;
        let initializer = match token.token_type {
            TokenType::Equal => {
                self.next();
                Some(self.expression()?)
            }
            _ => None,
        };

        self.next_if(|t| t == TokenTypeDiscriminants::Semicolon)
            .ok_or(ParseError::ExpectedToken(TokenType::Semicolon))?;
        Ok(Stmt::Var { name, initializer })
    }

    fn statement(&mut self) -> Result<Stmt> {
        let token = self.peek().ok_or(anyhow!("expected token"))?;

        self.next_if(|token| match token {
            TokenTypeDiscriminants::Print
            | TokenTypeDiscriminants::LeftBrace
            | TokenTypeDiscriminants::If
            | TokenTypeDiscriminants::For
            | TokenTypeDiscriminants::While => true,
            _ => false,
        });

        let stmt = match token.token_type {
            TokenType::For => {
                self.next_if(|t| t == TokenTypeDiscriminants::LeftParen)
                    .ok_or(ParseError::ExpectedToken(TokenType::LeftParen))?;

                let token = self.peek().ok_or(anyhow!("expected token"))?;
                let initializer: Option<Stmt> = match token.token_type.into() {
                    TokenTypeDiscriminants::Semicolon => {
                        self.next();
                        None
                    }
                    TokenTypeDiscriminants::Var => {
                        self.next();
                        Some(self.var_declaration()?)
                    }
                    _ => Some(Stmt::Expression(Box::new(self.expression()?))),
                };

                let token = self.peek().ok_or(anyhow!("expected token"))?;
                let condition: Expr = match token.token_type.into() {
                    TokenTypeDiscriminants::Semicolon => Expr::Literal(Object::Boolean(true)),
                    _ => self.expression()?,
                };
                self.next_if(|t| t == TokenTypeDiscriminants::Semicolon)
                    .ok_or(ParseError::ExpectedToken(TokenType::Semicolon))?;

                let token = self.peek().ok_or(anyhow!("expected token"))?;
                let increment: Option<Expr> = match token.token_type.into() {
                    TokenTypeDiscriminants::RightParen => None,
                    _ => Some(self.expression()?),
                };
                self.next_if(|t| t == TokenTypeDiscriminants::RightParen)
                    .ok_or(ParseError::ExpectedToken(TokenType::RightParen))?;

                let mut body = self.statement()?;

                if let Some(increment) = increment {
                    body = Stmt::Block(vec![
                        Box::new(body),
                        Box::new(Stmt::Expression(Box::new(increment))),
                    ]);
                }
                body = Stmt::While {
                    condition,
                    body: Box::new(body),
                };

                if let Some(initializer) = initializer {
                    body = Stmt::Block(vec![Box::new(initializer), Box::new(body)]);
                }

                body
            }
            TokenType::If => {
                self.next_if(|t| t == TokenTypeDiscriminants::LeftParen)
                    .ok_or(ParseError::ExpectedToken(TokenType::LeftParen))?;
                let condition = self.expression()?;
                self.next_if(|t| t == TokenTypeDiscriminants::RightParen)
                    .ok_or(ParseError::ExpectedToken(TokenType::RightParen))?;

                let then = Box::new(self.statement()?);
                let els = if self
                    .next_if(|t| t == TokenTypeDiscriminants::Else)
                    .is_some()
                {
                    Some(Box::new(self.statement()?))
                } else {
                    None
                };

                Stmt::If {
                    condition,
                    then,
                    els,
                }
            }
            TokenType::While => {
                self.next_if(|t| t == TokenTypeDiscriminants::LeftParen)
                    .ok_or(ParseError::ExpectedToken(TokenType::LeftParen));
                let condition = self.expression()?;
                self.next_if(|t| t == TokenTypeDiscriminants::RightParen)
                    .ok_or(ParseError::ExpectedToken(TokenType::RightParen));
                let body = self.statement()?;

                Stmt::While {
                    condition,
                    body: Box::new(body),
                }
            }
            TokenType::Print => {
                let value: Expr = self.expression()?;
                self.next_if(|t| t == TokenTypeDiscriminants::Semicolon)
                    .ok_or(ParseError::ExpectedToken(TokenType::Semicolon))?;
                Stmt::Print(Box::new(value))
            }
            TokenType::LeftBrace => Stmt::Block(self.block()?),
            _ => {
                let value: Expr = self.expression()?;
                self.next_if(|t| t == TokenTypeDiscriminants::Semicolon)
                    .ok_or(ParseError::ExpectedToken(TokenType::Semicolon))?;
                Stmt::Expression(Box::new(value))
            }
        };

        Ok(stmt)
    }

    fn block(&mut self) -> Result<Vec<Box<Stmt>>> {
        let mut statements = Vec::new();

        loop {
            let token = self.peek().ok_or(anyhow!("expected token"))?;
            match token.token_type {
                TokenType::Eof | TokenType::RightBrace => {
                    break;
                }
                _ => {
                    statements.push(self.declaration()?);
                }
            }
        }

        self.next_if(|t| t == TokenTypeDiscriminants::RightBrace)
            .ok_or(ParseError::ExpectedToken(TokenType::RightBrace))?;

        let statements = statements
            .into_iter()
            .flatten()
            .map(|s| Box::new(s))
            .collect();

        Ok(statements)
    }

    fn expression(&mut self) -> Result<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr> {
        let mut e = self.or()?;

        if let Some(token) = self.next_if(|t| t == TokenTypeDiscriminants::Equal) {
            let value = self.assignment()?;

            e = match e {
                Expr::Variable(v) => Ok(Expr::Assign {
                    name: v,
                    value: Box::new(value),
                }),
                _ => Err(anyhow!("invalid assigment target {}", token)),
            }?;
        };

        Ok(e)
    }

    fn or(&mut self) -> Result<Expr> {
        let mut expr = self.and()?;

        while let Some(operator) = self.next_if(|t| t == TokenTypeDiscriminants::Or) {
            let right = self.and()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr> {
        let mut expr = self.equality()?;

        while let Some(operator) = self.next_if(|t| t == TokenTypeDiscriminants::And) {
            let right = self.equality()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut e = self.comparison()?;

        while let Some(operator) = self.next_if(|t| {
            t == TokenTypeDiscriminants::BangEqual || t == TokenTypeDiscriminants::EqualEqual
        }) {
            let right = self.comparison()?;
            e = Expr::Binary {
                left: Box::new(e),
                operator,
                right: Box::new(right),
            };
        }

        Ok(e)
    }

    fn comparison(&mut self) -> Result<Expr> {
        let mut e = self.term()?;

        while let Some(operator) = self.next_if(|t| match t {
            TokenTypeDiscriminants::Greater
            | TokenTypeDiscriminants::GreaterEqual
            | TokenTypeDiscriminants::Less
            | TokenTypeDiscriminants::LessEqual => true,
            _ => false,
        }) {
            let right = self.term()?;
            e = Expr::Binary {
                left: Box::new(e),
                operator,
                right: Box::new(right),
            };
        }

        Ok(e)
    }

    fn term(&mut self) -> Result<Expr> {
        let mut e = self.factor()?;

        while let Some(operator) = self
            .next_if(|t| t == TokenTypeDiscriminants::Minus || t == TokenTypeDiscriminants::Plus)
        {
            let right = self.factor()?;
            e = Expr::Binary {
                left: Box::new(e),
                operator,
                right: Box::new(right),
            };
        }
        Ok(e)
    }

    fn factor(&mut self) -> Result<Expr> {
        let mut e = self.unary()?;

        while let Some(operator) = self
            .next_if(|t| t == TokenTypeDiscriminants::Slash || t == TokenTypeDiscriminants::Star)
        {
            let right = self.unary()?;
            e = Expr::Binary {
                left: Box::new(e),
                operator,
                right: Box::new(right),
            }
        }

        Ok(e)
    }

    fn unary(&mut self) -> Result<Expr> {
        if let Some(operator) = self
            .next_if(|t| t == TokenTypeDiscriminants::Bang || t == TokenTypeDiscriminants::Minus)
        {
            let right = self.unary()?;
            Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr> {
        // TODO: Could this be .next()?
        if let Some(token) = self.next_if(|_| true) {
            let literal = match token.token_type {
                TokenType::False => Expr::Literal(Object::Boolean(false)),
                TokenType::True => Expr::Literal(Object::Boolean(true)),
                TokenType::Nil => Expr::Literal(Object::Nil),
                TokenType::Number(n) => Expr::Literal(Object::Number(n)),
                TokenType::String(s) => Expr::Literal(Object::String(s)),
                TokenType::LeftParen => {
                    let e = self.expression()?;
                    self.next_if(|t| t == TokenTypeDiscriminants::RightParen)
                        .ok_or(ParseError::ExpectedToken(TokenType::RightParen))?;
                    Expr::Grouping {
                        grouping: Box::new(e),
                    }
                }
                TokenType::Identifier(_) => Expr::Variable(token),
                _ => return Err(anyhow!("dunno {}", token)),
            };

            Ok(literal)
        } else {
            unreachable!()
        }
    }

    fn next(&mut self) -> Option<Token> {
        self.current += 1;
        self.prev()
    }

    fn peek(&self) -> Option<Token> {
        match self.tokens.get(self.current) {
            Some(t) if t.token_type == TokenType::Eof => None,
            Some(t) => Some(t.clone()),
            None => None,
        }
    }

    fn prev(&self) -> Option<Token> {
        self.tokens.get(self.current - 1).cloned()
    }

    fn next_if<F>(&mut self, f: F) -> Option<Token>
    where
        F: FnOnce(TokenTypeDiscriminants) -> bool,
    {
        if self.peek().is_some_and(|token| f(token.token_type.into())) {
            self.next()
        } else {
            None
        }
    }

    #[allow(dead_code)] // TODO: Implemented as part of following the book, not used yet
    fn synchronize(&mut self) -> Result<()> {
        self.next();

        while let Some(token) = self.peek() {
            let prev = self.prev().ok_or(anyhow!("expected previous value"))?;
            if prev.token_type == TokenType::Semicolon {
                return Ok(());
            }

            match token.token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return Ok(()),
                _ => {}
            }
        }

        self.next();
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        data_types::Object,
        token::{Token, TokenType},
    };

    #[test]
    fn parse() {
        let tokens = vec![
            Token::new(TokenType::Minus, None, 1),
            Token::new(TokenType::Number(123 as f64), None, 1),
            Token::new(TokenType::Star, None, 1),
            Token::new(TokenType::LeftParen, None, 1),
            Token::new(TokenType::Number(45.67), None, 1),
            Token::new(TokenType::RightParen, None, 1),
            Token::new(TokenType::Semicolon, None, 1),
        ];

        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        let expected = Stmt::Expression(Box::new(Expr::Binary {
            left: Box::new(Expr::Unary {
                operator: Token::new(TokenType::Minus, None, 1),
                right: Box::new(Expr::Literal(Object::Number(123 as f64))),
            }),
            operator: Token::new(TokenType::Star, None, 1),
            right: Box::new(Expr::Grouping {
                grouping: Box::new(Expr::Literal(Object::Number(45.67))),
            }),
        }));

        // let expected = "(* (- 123) (group 45.67))";

        assert_eq!(expected, *expr.unwrap().first().unwrap());
    }
}
