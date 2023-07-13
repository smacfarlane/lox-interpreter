use crate::data_types;
use crate::token::Token;
use anyhow::Result;

pub(crate) trait Visitor<T> {
    fn visit_binary(&self, l: &Expr, o: &Token, r: &Expr) -> Result<T>;
    fn visit_grouping(&self, g: &Expr) -> Result<T>;
    fn visit_unary(&self, o: &Token, r: &Expr) -> Result<T>;
    fn visit_literal(&self, l: &data_types::Object) -> Result<T>;
}

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
    Literal(data_types::Object),
}

impl Expr {
    pub fn accept<V, T>(&self, visitor: V) -> Result<T>
    where
        V: Visitor<T>,
    {
        match self {
            Self::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary(left, operator, right),
            Self::Unary { operator, right } => visitor.visit_unary(operator, right),
            Self::Grouping { grouping } => visitor.visit_grouping(grouping),
            Self::Literal(literal) => visitor.visit_literal(literal),
        }
    }
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
            Expr::Literal(l) => write!(f, "{}", l),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data_types::Object;
    use crate::token::{Token, TokenType};

    #[test]
    fn display() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Unary {
                operator: Token::new(TokenType::Minus, None, 1),
                right: Box::new(Expr::Literal(Object::Number(123 as f64))),
            }),
            operator: Token::new(TokenType::Star, None, 1),
            right: Box::new(Expr::Grouping {
                grouping: Box::new(Expr::Literal(Object::Number(45.67))),
            }),
        };

        let expected = "(* (- 123) (group 45.67))";

        assert_eq!(expected, format!("{}", expr));
    }
}
