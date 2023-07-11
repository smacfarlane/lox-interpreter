use crate::{
    expr::Expr,
    token::{Token, TokenType},
};

use anyhow::{anyhow, Result};

#[derive(Debug)]
struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr> {
        self.equality()
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
                TokenType::False => Expr::False,
                TokenType::True => Expr::True,
                TokenType::Nil => Expr::Nil,
                TokenType::Number(n) => Expr::Number(n),
                TokenType::String(s) => Expr::String(s),
                TokenType::LeftParen => {
                    let e = self.expression()?;
                    self.consume(
                        TokenType::RightParen,
                        "Expected ')' after expression".to_string(),
                    );
                    Expr::Grouping {
                        grouping: Box::new(e),
                    }
                }
                _ => return Err(anyhow!("dunno")),
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
        self.tokens.get(self.current).cloned()
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
        match token.token_type {
            t => {
                self.next();
                Ok(())
            }
            _ => Err(anyhow!(message)),
        }
    }

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
    use crate::token::{Token, TokenType};

    #[test]
    fn parse() {
        let tokens = vec![
            Token::new(TokenType::Minus, None, 1),
            Token::new(TokenType::Number(123 as f64), None, 1),
            Token::new(TokenType::Star, None, 1),
            Token::new(TokenType::LeftParen, None, 1),
            Token::new(TokenType::Number(45.67), None, 1),
            Token::new(TokenType::RightParen, None, 1),
        ];

        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        let expected = Expr::Binary {
            left: Box::new(Expr::Unary {
                operator: Token::new(TokenType::Minus, None, 1),
                right: Box::new(Expr::Number(123 as f64)),
            }),
            operator: Token::new(TokenType::Star, None, 1),
            right: Box::new(Expr::Grouping {
                grouping: Box::new(Expr::Number(45.67)),
            }),
        };

        // let expr = Expr::Binary {
        //     left: Box::new(Expr::Unary {
        //         operator: Token::new(TokenType::Minus, None, 1),
        //         right: Box::new(Expr::Number(123 as f64)),
        //     }),
        //     operator: Token::new(TokenType::Star, None, 1),
        //     right: Box::new(Expr::Grouping {
        //         grouping: Box::new(Expr::Number(45.67)),
        //     }),
        // };

        // let expected = "(* (- 123) (group 45.67))";

        assert!(expr.is_ok());
        assert_eq!(expected, expr.unwrap());
    }
}
