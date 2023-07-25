use crate::token::Token;

pub enum Expr {
    Identifier(String),
    Literal(Token),
    Grouping(Box<Expr>),
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Assignment(String, Box<Expr>),
    Call(Box<Expr>, Token, Vec<Expr>),
}

impl Expr {
    pub fn accept<T>(&self, v: &mut dyn Visitor<T>) -> T {
        v.visit_expr(self)
    }
}

pub trait Visitor<T> {
    fn visit_expr(&mut self, expr: &Expr) -> T {
        unimplemented!();
    }

    fn visit_literal(&mut self, expr: &Expr, lit: &Token) -> T {
        self.visit_expr(expr)
    }

    fn visit_grouping(&mut self, expr: &Expr, inside: &Expr) -> T {
        self.visit_expr(expr)
    }

    fn visit_unary(&mut self, expr: &Expr, op: &Token, rhs: &Expr) -> T {
        self.visit_expr(expr)
    }
    fn visit_logical(&mut self, expr: &Expr, op: &Token, rhs: &Expr) -> T {
        self.visit_expr(expr)
    }
    fn visit_binary(&mut self, expr: &Expr, lhs: &Expr, op: &Token, rhs: &Expr) -> T {
        self.visit_expr(expr)
    }

    fn visit_identifier(&mut self, expr: &Expr, n: &String) -> T {
        self.visit_expr(expr)
    }

    fn visit_assignment(&mut self, expr: &Expr, n: &String, rhs: &Box<Expr>) -> T {
        self.visit_expr(expr)
    }

    fn visit_call(&mut self, expr: &Expr, paren: &Token, params: &[Expr]) -> T {
        self.visit_expr(expr)
    }
}

pub trait Boxed<T> {
    fn boxed(self) -> Box<T>;
}

impl Boxed<Expr> for Expr {
    fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}
