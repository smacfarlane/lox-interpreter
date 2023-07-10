use super::*;
use crate::token::Token;

#[derive(Clone, Debug)]
pub(crate) struct Binary<E, F>
where
    E: Expr,
    F: Expr,
{
    pub(crate) left: E,
    pub(crate) operator: Token,
    pub(crate) right: F,
}

impl<E, F> Expr for Binary<E, F>
where
    E: Expr,
    F: Expr,
{
}

impl<E, F> std::fmt::Display for Binary<E, F>
where
    E: Expr + std::fmt::Display,
    F: Expr + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({} {} {})", self.operator, self.left, self.right)
    }
}
