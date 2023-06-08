use crate::token::{Literal, Token, TokenType};

#[derive(Debug)]
pub enum Expr {
    Literal(Token),
    Grouping(Box<Expr>),
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
}

impl Expr {
    pub fn accept<T>(&self, v: &mut dyn Visitor<T>) -> T {
        match *self {
            Expr::Literal(ref lit) => v.visit_literal(self, lit),
            Expr::Grouping(ref inside) => v.visit_grouping(self, inside.as_ref()),
            Expr::Unary(ref op, ref rhs) => v.visit_unary(self, op, rhs.as_ref()),
            Expr::Binary(ref lhs, ref op, ref rhs) => {
                v.visit_binary(self, lhs.as_ref(), op, rhs.as_ref())
            }
        }
    }
}

/*
#[derive(Debug)]
pub enum ExprType {
    Binary(Binary),
    Unary(Unary),
    Grouping(Box<Expr>),
    Literal(Literal),
}

impl ExprType {
    pub fn to_str(&self) -> &str {
        match self {
            ExprType::Logical(l) => l.to_str(),
            ExprType::Binary(b) => b.to_str(),
            ExprType::Unary(u) => u.to_str(),
            ExprType::Literal(l) => l,
        }
    }
}


impl ExprType {
    pub fn binary(left: Expr, operator: BinaryOperator, right: Expr) -> ExprType {
        ExprType::Binary(Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    pub fn unary(operator: UnaryOperator, unary: Expr) -> ExprType {
        ExprType::Unary(Unary {
            operator,
            unary: Box::new(unary),
        })
    }
    pub fn grouping(expr: Expr) -> ExprType {
        ExprType::Grouping(Box::new(expr))
    }
}

#[derive(Debug)]
pub enum Literal {
    String(String),
    Number(f64),
    True,
    False,
    None,
}

#[derive(Debug)]
pub enum BinaryOperator {
    Equal,
    BangEq,
    GreaterThan,
    GreaterThanEq,
    LessThan,
    LessThanEq,
    Minus,
    Plus,
    Slash,
    Star,
    And,
    Or,
}

impl BinaryOperator {
    pub fn to_str(&self) -> &str {
        match self {
            BinaryOperator::Equal => "=",
            BinaryOperator::BangEq => "!=",
            BinaryOperator::GreaterThan => ">",
            BinaryOperator::GreaterThanEq => ">=",
            BinaryOperator::LessThan => "<",
            BinaryOperator::LessThanEq => "<=",
            BinaryOperator::Minus => "-",
            BinaryOperator::Plus => "+",
            BinaryOperator::Slash => "/",
            BinaryOperator::Star => "*",
            BinaryOperator::Or => "or",
            BinaryOperator::And => "and",
        }
    }
}

#[derive(Debug)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: BinaryOperator,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub enum UnaryOperator {
    Bang,
    Minus,
}

impl UnaryOperator {
    pub fn to_str(&self) -> &str {
        match self {
            UnaryOperator::Bang => "!",
            UnaryOperator::Minus => "-",
        }
    }
}

#[derive(Debug)]
pub struct Unary {
    operator: UnaryOperator,
    unary: Box<Expr>,
}

*/

pub fn print_ast(expr: Expr) {
    println!("{:#?}", expr);
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

    fn visit_binary(&mut self, expr: &Expr, lhs: &Expr, op: &Token, rhs: &Expr) -> T {
        self.visit_expr(expr)
    }
}
