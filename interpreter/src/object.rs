use crate::token::Literal;

#[derive(Debug)]
pub enum Object {
    Literal(Literal),
}
