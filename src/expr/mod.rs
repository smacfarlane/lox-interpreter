mod binary;
mod grouping;
mod literal;
mod unary;

// enum Expr {
//     Binary {
//         left: Box<Expr>,
//         operator: Token,
//         right: Box<Expr>,
//     },
//     Grouping {
//         grouping: Box<Expr>,
//     },
//     Literal(dyn std::any::Any),
//     Unary {
//         operator: Token,
//         right: Box<Expr>,
//     },
// }

pub(crate) trait Expr {}

// pub(crate) trait Visitor<E, T>
// where
//     E: Expr,
// {
//     fn visitBinary(b: binary::Binary<E>);
//     fn visitGrouping(g: grouping::Grouping<E>);
//     fn visitLiteral(l: literal::Literal<T>);
//     fn visitUnary(u: unary::Unary<E>);
// }

#[cfg(test)]
mod test {
    use super::*;
    use crate::token::{Token, TokenType};

    #[test]
    fn display() {
        let expr = binary::Binary {
            left: unary::Unary {
                operator: Token::new(TokenType::Minus, None, 1),
                right: literal::Literal { literal: 123 },
            },
            operator: Token::new(TokenType::Star, None, 1),
            right: grouping::Grouping {
                grouping: literal::Literal { literal: 45.67 },
            },
        };

        let expected = "(* (- 123) (group 45.67))";

        assert_eq!(expected, format!("{}", expr));
    }
}
