use crate::data_types;
use crate::token::Token;
use anyhow::Result;

pub trait ExpressionVisitor<T> {
    fn visit_assignment(&mut self, t: &Token, e: &Expr) -> Result<T>;
    fn visit_binary(&mut self, l: &Expr, o: &Token, r: &Expr) -> Result<T>;
    fn visit_grouping(&mut self, g: &Expr) -> Result<T>;
    fn visit_unary(&mut self, o: &Token, r: &Expr) -> Result<T>;
    fn visit_literal(&mut self, l: &data_types::Object) -> Result<T>;
    fn visit_variable(&mut self, n: &Token) -> Result<T>;
}

pub trait StatementVisitor {
    fn visit_print(&mut self, e: &Expr) -> Result<()>;
    fn visit_expression(&mut self, e: &Expr) -> Result<()>;
    fn visit_variable(&mut self, n: &Token, i: Option<&Expr>) -> Result<()>;
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        grouping: Box<Expr>,
    },
    Literal(data_types::Object),
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable(Token),
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expression(Box<Expr>),
    Print(Box<Expr>),
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
}

impl Stmt {
    pub fn accept<V>(&self, visitor: &mut V) -> Result<()>
    where
        V: StatementVisitor,
    {
        match self {
            Self::Print(expr) => visitor.visit_print(expr), // TODO: This should be a Stmt::Print
            Self::Expression(expr) => visitor.visit_expression(expr),
            Self::Var {
                name: name,
                initializer: init,
            } => visitor.visit_variable(name, init.as_ref()),
        }
    }
}

impl Expr {
    pub fn accept<V, T>(&self, visitor: &mut V) -> Result<T>
    where
        V: ExpressionVisitor<T>,
    {
        match self {
            Self::Assign { name, value } => visitor.visit_assignment(name, value),
            Self::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary(left, operator, right),
            Self::Grouping { grouping } => visitor.visit_grouping(grouping),
            Self::Literal(literal) => visitor.visit_literal(literal),
            Self::Unary { operator, right } => visitor.visit_unary(operator, right),
            Self::Variable(name) => visitor.visit_variable(name),
        }
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expr::Assign { name, value } => write!(f, "({} = {})", name, value),
            Expr::Binary {
                left,
                operator,
                right,
            } => write!(f, "({} {} {})", operator, left, right),
            Expr::Unary { operator, right } => write!(f, "({} {})", operator, right),
            Expr::Grouping { grouping } => write!(f, "(group {})", grouping),
            Expr::Literal(l) => write!(f, "{}", l),
            Expr::Variable(v) => write!(f, "{}", v),
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
