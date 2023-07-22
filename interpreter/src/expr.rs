use crate::token::Token;

#[derive(Debug)]
pub enum Expr {
    Identifier(String),
    Literal(Token),
    Grouping(Box<Expr>),
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Assignment(String, Box<Expr>),
}

impl Expr {
    pub fn accept<T>(&self, v: &mut dyn Visitor<T>) -> T {
        v.visit_expr(self)
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
}

pub trait Boxed<T> {
    fn boxed(self) -> Box<T>;
}

impl Boxed<Expr> for Expr {
    fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}
