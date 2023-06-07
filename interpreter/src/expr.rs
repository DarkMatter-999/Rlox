use crate::token::Token;

#[derive(Debug)]
pub struct Expr {
    pub node: ExprType,
}

impl Expr {
    pub fn new(node: ExprType) -> Self {
        Expr { node }
    }
}

#[derive(Debug)]
pub enum ExprType {
    Logical(Logical),
    Binary(Binary),
    Unary(Unary),
    Grouping(Box<Expr>),
    Literal(Literal),
}

/*
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
*/

impl ExprType {
    pub fn binary(left: Expr, operator: BinaryOperator, right: Expr) -> ExprType {
        ExprType::Binary(Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    pub fn logical(left: Expr, operator: LogicalOperator, right: Expr) -> ExprType {
        ExprType::Logical(Logical {
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
pub enum LogicalOperator {
    And,
    Or,
}

impl LogicalOperator {
    pub fn to_str(&self) -> &str {
        match self {
            LogicalOperator::And => "and",
            LogicalOperator::Or => "or",
        }
    }
}

#[derive(Debug)]
pub struct Logical {
    pub left: Box<Expr>,
    pub operator: LogicalOperator,
    pub right: Box<Expr>,
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

pub fn print_ast(expr: Expr) {
    println!("{:#?}", expr);
}
