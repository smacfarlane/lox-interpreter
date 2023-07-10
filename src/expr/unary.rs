use super::*;
use crate::token::Token;

#[derive(Debug)]
pub(crate) struct Unary<E>
where
    E: Expr,
{
    pub(crate) operator: Token,
    pub(crate) right: E,
}

impl<E> Expr for Unary<E> where E: Expr {}

impl<E> std::fmt::Display for Unary<E>
where
    E: Expr + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({} {})", self.operator, self.right)
    }
}
