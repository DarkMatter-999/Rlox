use crate::{
    expr::{Expr, Visitor},
    object::Object,
    token::{Literal, Token, TokenType},
};

pub struct Interpreter {
    pub tokens: Vec<Token>,
}

impl Interpreter {
    pub fn new(tokens: Vec<Token>) -> Self {
        Interpreter { tokens }
    }
}
