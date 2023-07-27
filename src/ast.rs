use crate::data_types::{Object, Return};
use crate::token::Token;
use anyhow::Result;

pub trait ExpressionVisitor<T> {
    fn visit_assignment(&mut self, t: &Token, e: &Expr) -> Result<T>;
    fn visit_binary(&mut self, l: &Expr, o: &Token, r: &Expr) -> Result<T>;
    fn visit_call(&mut self, c: &Expr, p: &Token, a: &Vec<Box<Expr>>) -> Result<T>;
    fn visit_grouping(&mut self, g: &Expr) -> Result<T>;
    fn visit_unary(&mut self, o: &Token, r: &Expr) -> Result<T>;
    fn visit_literal(&mut self, l: &Object) -> Result<T>;
    fn visit_logical(&mut self, l: &Expr, o: &Token, r: &Expr) -> Result<T>;
    fn visit_variable(&mut self, n: &Token) -> Result<T>;
}

pub trait StatementVisitor {
    fn visit_block(&mut self, s: &Vec<Box<Stmt>>) -> Result<Return>;
    fn visit_if(&mut self, c: &Expr, t: &Stmt, e: Option<&Stmt>) -> Result<Return>;
    fn visit_print(&mut self, e: &Expr) -> Result<Return>;
    fn visit_expression(&mut self, e: &Expr) -> Result<Return>;
    fn visit_variable(&mut self, n: &Token, i: Option<&Expr>) -> Result<Return>;
    fn visit_while(&mut self, c: &Expr, o: &Stmt) -> Result<Return>;
    fn visit_function(&mut self, n: &Token, p: &Vec<Token>, b: &Vec<Box<Stmt>>) -> Result<Return>;
    fn visit_return(&mut self, t: &Token, e: Option<&Expr>) -> Result<Return>;
}

#[derive(Clone, Debug, PartialEq)]
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
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Box<Expr>>,
    },
    Grouping {
        grouping: Box<Expr>,
    },
    Literal(Object),
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable(Token),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Block(Vec<Box<Stmt>>),
    If {
        condition: Expr,
        then: Box<Stmt>,
        els: Option<Box<Stmt>>,
    },
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Box<Stmt>>,
    },
    Expression(Box<Expr>),
    Print(Box<Expr>),
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    Return {
        token: Token,
        value: Option<Expr>,
    },
}

impl Stmt {
    pub fn accept<V>(&self, visitor: &mut V) -> Result<Return>
    where
        V: StatementVisitor,
    {
        match self {
            Self::Block(stmts) => visitor.visit_block(stmts),
            Self::If {
                condition,
                then,
                els,
            } => visitor.visit_if(condition, then, els.as_deref()),
            Self::Print(expr) => visitor.visit_print(expr), // TODO: This should be a Stmt::Print (why?)
            Self::Expression(expr) => visitor.visit_expression(expr),
            Self::Function { name, params, body } => visitor.visit_function(name, params, body),
            Self::Return { token, value } => visitor.visit_return(token, value.as_ref()),
            Self::Var {
                name: name,
                initializer: init,
            } => visitor.visit_variable(name, init.as_ref()),
            Self::While { condition, body } => visitor.visit_while(condition, body),
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
            Self::Call {
                callee,
                paren,
                arguments,
            } => visitor.visit_call(callee, paren, arguments),
            Self::Grouping { grouping } => visitor.visit_grouping(grouping),
            Self::Literal(literal) => visitor.visit_literal(literal),
            Self::Logical {
                left,
                operator,
                right,
            } => visitor.visit_logical(left, operator, right),
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
            }
            | Expr::Logical {
                left,
                operator,
                right,
            } => write!(f, "({} {} {})", operator, left, right),
            Expr::Call { callee, .. } => write!(f, "({} (arguments))", callee), // TODO: Clean this up
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
