use crate::{expr::Expr, token::Token};

#[derive(Debug)]
pub enum Stmt {
    Empty,
    Expression(Expr),
    Print(Expr),
    Declaration(String, Option<Expr>),
    Block(Vec<Stmt>),
}

impl Stmt {
    pub fn accept<T>(&self, v: &mut dyn Visitor<T>) -> T {
        v.visit_stmt(self)
    }
}

pub trait Visitor<T> {
    fn visit_stmt(&mut self, e: &Stmt) -> T;

    fn visit_block_stmt(&mut self, statements: &Vec<Stmt>) -> T;

    fn visit_expression_stmt(&mut self, expression: &Expr) -> T;

    fn visit_print_stmt(&mut self, expression: &Expr) -> T;

    fn visit_declaration_stmt(&mut self, name: &String, init: Option<&Expr>) -> T;
}
