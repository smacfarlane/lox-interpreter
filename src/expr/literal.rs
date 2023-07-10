use super::*;

#[derive(Clone, Debug)]
pub(crate) struct Literal<T>
where
    T: Clone,
{
    pub(crate) literal: T,
}

impl<T> Expr for Literal<T> where T: Clone {}

impl<T> std::fmt::Display for Literal<T>
where
    T: Clone + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.literal)
    }
}
