use crate::ast::{Expr, ExpressionVisitor, StatementVisitor, Stmt};
use crate::data_types::Return;
use crate::token::Token;

use std::collections::HashMap;

use anyhow::anyhow;

#[derive(Debug)]
pub struct Resolver /*<'a>*/ {
    // interpreter: &'a mut Interpreter,
    locals: Locals,
    scopes: Vec<Scope>,
}

#[derive(Debug, Clone)]
pub struct Locals {
    locals: HashMap<usize, Vec<Expr>>,
}

impl Locals {
    pub fn new() -> Self {
        Locals {
            locals: HashMap::new(),
        }
    }

    // pub fn insert(&mut self, depth: usize, expr: Expr) {
    //     self.locals
    //         .entry(depth)
    //         .and_modify(|d| {
    //             d.push(expr);
    //         })
    //         .or_insert(vec![expr]);
    // }
}

type Scope = HashMap<String, bool>;

impl Resolver {
    //<'_> {
    // pub fn new(interpreter: &'bmut Interpreter) -> Resolver<'b> {
    pub fn new() -> Resolver {
        Resolver {
            // interpreter,
            scopes: vec![],
            locals: Locals::new(),
        }
    }

    pub fn locals(&self) -> Locals {
        self.locals.clone()
    }

    fn begin_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) -> anyhow::Result<()> {
        let name = name
            .lexeme
            .clone()
            .ok_or(anyhow!("attempted to declare variable without name"))?;
        if !self.scopes.is_empty() {
            if self.scopes.last().unwrap().contains_key(&name) {
                return Err(anyhow!(
                    "'{}' has already been declared in this scope",
                    name
                ));
            }
            self.scopes.last_mut().unwrap().insert(name, false); // Safety: scopes is not empty
        }

        Ok(())
    }

    fn define(&mut self, name: &Token) -> anyhow::Result<()> {
        let name = name
            .lexeme
            .clone()
            .ok_or(anyhow!("attempted to define variable without name"))?;
        if !self.scopes.is_empty() {
            self.scopes.last_mut().unwrap().insert(name, true); // Safety: scopes is not empty
        }

        Ok(())
    }

    fn resolve_expr(&mut self, expr: &Expr) -> anyhow::Result<()> {
        expr.accept::<Resolver, ()>(self)
    }

    pub fn resolve_stmt(&mut self, stmt: &Stmt) -> anyhow::Result<()> {
        stmt.accept::<Resolver>(self)?;
        Ok(())
    }

    fn resolve_local(&mut self, expr: &Expr) -> anyhow::Result<()> {
        // token: &Token, value: Option<&Expr>)
        let lexeme = match expr {
            Expr::Variable(token) => token.lexeme.clone().unwrap(),
            Expr::Assign { name, value } => name.lexeme.clone().unwrap(),
            _ => unreachable!(),
        };
        // let lexeme = token.lexeme.clone().unwrap();
        for (depth, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&lexeme) {
                // self.locals.insert(depth, expr.clone());
                // self.interpreter.resolve(expr.clone(), depth);
                return Ok(());
            }
        }

        Ok(())
    }

    fn resolve_function(
        &mut self,
        _: &Token,
        params: &Vec<Token>,
        body: &Vec<Box<Stmt>>,
    ) -> anyhow::Result<()> {
        self.begin_scope();
        for param in params.iter() {
            self.declare(param)?;
            self.define(param)?;
        }

        for stmt in body {
            self.resolve_stmt(&stmt)?;
        }

        self.end_scope();

        Ok(())
    }
}

impl StatementVisitor for Resolver /* TODO <'_> */ {
    fn visit_block(&mut self, s: &Vec<Box<Stmt>>) -> anyhow::Result<Return> {
        self.begin_scope();
        for stmt in s {
            self.resolve_stmt(stmt)?;
        }
        self.end_scope();

        Ok(Return::None)
    }

    fn visit_expression(&mut self, e: &Expr) -> anyhow::Result<Return> {
        self.resolve_expr(e)?;
        Ok(Return::None)
    }

    fn visit_if(
        &mut self,
        condition: &Expr,
        thn: &Stmt,
        els: Option<&Stmt>,
    ) -> anyhow::Result<Return> {
        self.resolve_expr(condition)?;
        self.resolve_stmt(thn)?;

        if let Some(els) = els {
            self.resolve_stmt(els)?;
        }

        Ok(Return::None)
    }

    fn visit_function(
        &mut self,
        n: &Token,
        p: &Vec<Token>,
        b: &Vec<Box<Stmt>>,
    ) -> anyhow::Result<Return> {
        self.declare(n)?;
        self.define(n)?;

        self.resolve_function(n, p, b)?;

        Ok(Return::None)
    }

    fn visit_print(&mut self, e: &Expr) -> anyhow::Result<Return> {
        self.resolve_expr(e)?;

        Ok(Return::None)
    }

    fn visit_return(&mut self, _: &Token, value: Option<&Expr>) -> anyhow::Result<Return> {
        if let Some(value) = value {
            self.resolve_expr(value)?;
        }

        Ok(Return::None)
    }

    fn visit_variable(&mut self, n: &Token, i: Option<&Expr>) -> anyhow::Result<Return> {
        self.declare(n)?;

        if let Some(i) = i {
            self.resolve_expr(i)?;
        }

        self.define(n)?;

        Ok(Return::None)
    }

    fn visit_while(&mut self, condition: &Expr, body: &Stmt) -> anyhow::Result<Return> {
        self.resolve_expr(condition)?;
        self.resolve_stmt(body)?;

        Ok(Return::None)
    }
}

impl ExpressionVisitor<()> for Resolver /*<'_>*/ {
    fn visit_assignment(&mut self, name: &Token, value: &Expr) -> anyhow::Result<()> {
        self.resolve_expr(value)?;
        self.resolve_local(&Expr::Assign {
            name: name.clone(),
            value: Box::new(value.clone()),
        })?;

        Ok(())
    }

    fn visit_binary(&mut self, left: &Expr, _: &Token, right: &Expr) -> anyhow::Result<()> {
        self.resolve_expr(left)?;
        self.resolve_expr(right)?;

        Ok(())
    }

    fn visit_call(
        &mut self,
        callee: &Expr,
        _: &Token,
        args: &Vec<Box<Expr>>,
    ) -> anyhow::Result<()> {
        self.resolve_expr(callee)?;

        for arg in args {
            self.resolve_expr(arg)?;
        }

        Ok(())
    }

    fn visit_grouping(&mut self, grouping: &Expr) -> anyhow::Result<()> {
        self.resolve_expr(grouping)?;

        Ok(())
    }

    fn visit_literal(&mut self, _: &crate::data_types::Object) -> anyhow::Result<()> {
        Ok(())
    }

    fn visit_logical(&mut self, left: &Expr, _: &Token, right: &Expr) -> anyhow::Result<()> {
        self.resolve_expr(left)?;
        self.resolve_expr(right)?;

        Ok(())
    }

    fn visit_unary(&mut self, _: &Token, right: &Expr) -> anyhow::Result<()> {
        self.resolve_expr(right)?;

        Ok(())
    }

    fn visit_variable(&mut self, n: &Token) -> anyhow::Result<()> {
        let name = n.lexeme.clone().unwrap();
        if !self.scopes.is_empty() && self.scopes.last().unwrap().get(&name) == Some(&false) {
            return Err(anyhow!("can't read local variable in its own initializer"));
        }

        self.resolve_local(&Expr::Variable(n.clone()))
    }
}
