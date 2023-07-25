use std::rc::Rc;

use crate::expr::{Boxed, Expr};

pub enum Stmt {
    Empty,
    Break(u32),
    Expression(Expr),
    Print(Expr),
    Declaration(String, Option<Expr>),
    Block(Vec<Stmt>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    While(Expr, Box<Stmt>),
    Function(String, Vec<String>, Rc<Stmt>),
    Return(u64, Expr),
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

    fn visit_if(&mut self, expr: &Expr, then_stmt: &Stmt, else_stmt: Option<&Stmt>) -> T;

    fn visit_while(&mut self, expr: &Expr, body: &Stmt) -> T;

    fn visit_break(&mut self, line: u32) -> T;

    fn visit_function(&mut self, name: String, params: &[String], body: Rc<Stmt>) -> T;

    fn visit_return(&mut self, line: u64, expr: &Expr) -> T;
}

impl Boxed<Stmt> for Stmt {
    fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}
