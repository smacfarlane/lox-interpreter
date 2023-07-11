use crate::token::Token;

#[derive(Debug, PartialEq)]
pub(crate) enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        grouping: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Number(f64),
    String(String),
    True,
    False,
    Nil,
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => write!(f, "({} {} {})", operator, left, right),
            Expr::Unary { operator, right } => write!(f, "({} {})", operator, right),
            Expr::Grouping { grouping } => write!(f, "(group {})", grouping),
            Expr::Number(n) => write!(f, "{}", n),
            Expr::String(s) => write!(f, "{}", s),
            Expr::True => write!(f, "true"),
            Expr::False => write!(f, "false"),
            Expr::Nil => write!(f, "nil"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::token::{Token, TokenType};

    #[test]
    fn display() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Unary {
                operator: Token::new(TokenType::Minus, None, 1),
                right: Box::new(Expr::Number(123 as f64)),
            }),
            operator: Token::new(TokenType::Star, None, 1),
            right: Box::new(Expr::Grouping {
                grouping: Box::new(Expr::Number(45.67)),
            }),
        };

        let expected = "(* (- 123) (group 45.67))";

        assert_eq!(expected, format!("{}", expr));
    }
}
