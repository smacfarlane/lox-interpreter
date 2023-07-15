use crate::{
    ast::{Expr, Stmt},
    data_types::Object,
    token::{Token, TokenType},
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

        self.consume(
            TokenType::Semicolon,
            "expected ';' after variable initialization".to_string(),
        )?;
        Ok(Stmt::Var { name, initializer })
    }

    fn statement(&mut self) -> Result<Stmt> {
        let token = self.peek().ok_or(anyhow!("expected token"))?;

        match token.token_type {
            TokenType::Print => {
                self.next().ok_or(anyhow!("expected token"))?;
            }
            _ => {}
        }

        let value: Expr = self.expression()?;
        self.consume(TokenType::Semicolon, "expected ';' after value".to_string())?;

        let stmt = match token.token_type {
            TokenType::Print => Stmt::Print(Box::new(value)),
            _ => Stmt::Expression(Box::new(value)),
        };
        dbg!(&stmt);

        Ok(stmt)
    }

    fn expression(&mut self) -> Result<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr> {
        let mut e = self.equality()?;

        if self.matches_next(vec![TokenType::Equal]) {
            let equals = self.prev().ok_or(anyhow!("expected previous token"))?;
            let value = self.assignment()?;

            e = match e {
                Expr::Variable(v) => Ok(Expr::Assign {
                    name: v,
                    value: Box::new(value),
                }),
                _ => Err(anyhow!("invalid assigment target")),
            }?;
        }

        Ok(e)
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut e = self.comparison()?;

        while self.matches_next(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.prev().ok_or(anyhow!("no previous token"))?;
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

        while self.matches_next(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.prev().ok_or(anyhow!("no previous token"))?;
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

        while self.matches_next(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.prev().unwrap();
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

        while self.matches_next(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.prev().unwrap();
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
        if let Some(token) = self.peek() {
            match token.token_type {
                TokenType::Bang | TokenType::Minus => {
                    self.next();
                    let operator = token;
                    let right = self.unary()?;
                    Ok(Expr::Unary {
                        operator,
                        right: Box::new(right),
                    })
                }
                _ => self.primary(),
            }
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr> {
        if let Some(token) = self.peek() {
            self.next();
            let literal = match token.token_type {
                TokenType::False => Expr::Literal(Object::Boolean(false)),
                TokenType::True => Expr::Literal(Object::Boolean(true)),
                TokenType::Nil => Expr::Literal(Object::Nil),
                TokenType::Number(n) => Expr::Literal(Object::Number(n)),
                TokenType::String(s) => Expr::Literal(Object::String(s)),
                TokenType::LeftParen => {
                    let e = self.expression()?;
                    self.consume(
                        TokenType::RightParen,
                        "Expected ')' after expression".to_string(),
                    )?;
                    Expr::Grouping {
                        grouping: Box::new(e),
                    }
                }
                TokenType::Identifier(_) => Expr::Variable(token),
                _ => return Err(anyhow!("dunno {}", dbg!(token))),
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
        // self.tokens.get(self.current).cloned()
    }

    fn prev(&self) -> Option<Token> {
        self.tokens.get(self.current - 1).cloned()
    }

    fn matches_next(&mut self, token_types: Vec<TokenType>) -> bool {
        for tt in token_types {
            if let Some(tok) = self.peek() {
                if tok.token_type == tt {
                    self.next();
                    return true;
                }
            }
        }
        false
    }

    fn consume(&mut self, t: TokenType, message: String) -> Result<()> {
        let token = self.peek().ok_or(anyhow!(message.clone()))?;
        if token.token_type == t {
            self.next();
            Ok(())
        } else {
            Err(anyhow!(message))
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
