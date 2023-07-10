use super::*;

pub(crate) struct Grouping<E>
where
    E: Expr,
{
    pub(crate) grouping: E,
}

impl<E> Expr for Grouping<E> where E: Expr {}

impl<E> std::fmt::Display for Grouping<E>
where
    E: Expr + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(group {})", self.grouping)
    }
}
