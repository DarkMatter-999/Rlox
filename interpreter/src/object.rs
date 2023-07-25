use crate::{function::Callable, token::Literal};
use std::{
    cmp::{self, Ordering},
    fmt,
    rc::Rc,
};

#[derive(Clone)]
pub enum Object {
    Literal(Literal),
    Func(Rc<dyn Callable>),
}

impl Object {
    pub fn is_truthy(&self) -> bool {
        match self {
            Object::Literal(l) => match l {
                Literal::True => return true,
                Literal::False => return false,
                Literal::None => return false,
                Literal::Number(n) => return *n != 0.0,
                Literal::StringLit(s) => return !s.is_empty(),
            },
            Object::Func(_) => true,
        }
    }
}

impl cmp::PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Object::Literal(ref lhs), &Object::Literal(ref rhs)) => lhs.eq(rhs),

            _ => false,
        }
    }
}

impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (&Object::Literal(ref l), &Object::Literal(ref r)) => l.partial_cmp(r),
            _ => None,
        }
    }
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Object::Literal(ref lit) => lit.fmt(f),
            Object::Func(_) => write!(f, "<function>"),
        }
    }
}
