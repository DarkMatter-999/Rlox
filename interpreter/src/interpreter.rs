use crate::{
    error::{Error, ResultMSG},
    expr::{Expr, Visitor},
    object::Object,
    token::{Literal, Token, TokenType},
};

pub struct Interpreter {}

impl Interpreter {
    pub fn evaluate(&mut self, expr: &Expr) -> ResultMSG<Object> {
        self.visit_grouping(expr, expr)
    }
}

impl Visitor<ResultMSG<Object>> for Interpreter {
    fn visit_literal(&mut self, expr: &Expr, lit: &Token) -> ResultMSG<Object> {
        Ok(Object::Literal(lit.lexeme.clone()))
    }

    fn visit_grouping(&mut self, expr: &Expr, inside: &Expr) -> ResultMSG<Object> {
        inside.accept(self)
    }

    fn visit_unary(&mut self, expr: &Expr, op: &Token, rhs: &Expr) -> ResultMSG<Object> {
        let r: Object = rhs.accept(self)?;

        match op.token_type {
            TokenType::MINUS => match r {
                Object::Literal(Literal::Number(n)) => Ok(Object::Literal(Literal::Number(-n))),
                _ => self.err_near("cannot negate non-numeric", op, format!("{:?}", r)),
            },
            TokenType::BANG => match !r.is_truthy() {
                true => Ok(Object::Literal(Literal::True)),
                false => Ok(Object::Literal(Literal::False)),
            },
            _ => self.err_op("erroneous unary operator", op),
        }
    }

    fn visit_binary(
        &mut self,
        _expr: &Expr,
        lhs: &Expr,
        op: &Token,
        rhs: &Expr,
    ) -> ResultMSG<Object> {
        use crate::object::Object::Literal as ObjLit;
        use crate::token::Literal::{Number, StringLit};
        use std::cmp::Ordering as Ord;

        if op.token_type == TokenType::OR || op.token_type == TokenType::AND {
            return self.visit_logical(lhs, op, rhs);
        }

        let l: Object = lhs.accept(self)?;
        let r: Object = rhs.accept(self)?;

        let res: Literal = match op.token_type {
            TokenType::PLUS => match (l, r) {
                (ObjLit(Number(ref ln)), ObjLit(Number(ref rn))) => Number(ln + rn),
                (ObjLit(StringLit(ref ln)), ObjLit(ref r)) => StringLit(format!("{}{:?}", ln, r)),
                (ObjLit(ref l), ObjLit(StringLit(ref rn))) => StringLit(format!("{:?}{}", l, rn)),
                (ref l, ref r) => {
                    return self.err_near(
                        "cannot add mixed types",
                        op,
                        format!("{:?} + {:?}", l, r),
                    )
                }
            },
            TokenType::MINUS => match (l, r) {
                (ObjLit(Number(ln)), ObjLit(Number(rn))) => Number(ln - rn),
                (l, r) => {
                    return self.err_near(
                        "cannot subtract non-numerics",
                        op,
                        format!("{:?} - {:?}", l, r),
                    )
                }
            },
            TokenType::STAR => match (l, r) {
                (ObjLit(Number(ln)), ObjLit(Number(rn))) => Number(ln * rn),
                (l, r) => {
                    return self.err_near(
                        "cannot multiply non-numerics",
                        op,
                        format!("{:?} * {:?}", l, r),
                    )
                }
            },
            TokenType::SLASH => match (l, r) {
                (ObjLit(Number(ln)), ObjLit(Number(rn))) if rn == 0.0 => {
                    return self.err_near("divide by zero", op, format!("{:?} / {:?}", ln, rn))
                }
                (ObjLit(Number(ln)), ObjLit(Number(rn))) => Number(ln / rn),
                (l, r) => {
                    return self.err_near(
                        "cannot multiply non-numerics",
                        op,
                        format!("{:?} * {:?}", l, r),
                    )
                }
            },
            TokenType::GREATER
            | TokenType::GREATER_EQUAL
            | TokenType::LESS
            | TokenType::LESS_EQUAL => match l.partial_cmp(&r) {
                Some(Ord::Less) => {
                    if op.in_types(vec![TokenType::LESS, TokenType::LESS_EQUAL]) {
                        Literal::True
                    } else {
                        Literal::False
                    }
                }
                Some(Ord::Equal) => {
                    if op.in_types(vec![TokenType::LESS_EQUAL, TokenType::GREATER_EQUAL]) {
                        Literal::True
                    } else {
                        Literal::False
                    }
                }
                Some(Ord::Greater) => {
                    if op.in_types(vec![TokenType::GREATER, TokenType::GREATER_EQUAL]) {
                        Literal::True
                    } else {
                        Literal::False
                    }
                }
                None => {
                    return self.err_near("cannot compare types", op, format!("{:?} ? {:?}", l, r))
                }
            },
            TokenType::EQUAL_EQUAL => {
                if l.eq(&r) {
                    Literal::True
                } else {
                    Literal::False
                }
            }
            TokenType::BANG_EQUAL => {
                if l.ne(&r) {
                    Literal::True
                } else {
                    Literal::False
                }
            }
            _ => return self.err_op("erroneous binary operator", op),
        };

        Ok(ObjLit(res))
    }
    fn visit_logical(&mut self, lhs: &Expr, op: &Token, rhs: &Expr) -> ResultMSG<Object> {
        let l: Object = lhs.accept(self)?;

        let res: Literal = match op.token_type {
            TokenType::AND if l.is_truthy() => {
                if rhs.accept(self)?.is_truthy() {
                    Literal::True
                } else {
                    Literal::False
                }
            }
            TokenType::OR if l.is_truthy() => Literal::True,
            TokenType::OR => {
                if rhs.accept(self)?.is_truthy() {
                    Literal::True
                } else {
                    Literal::False
                }
            }
            _ => Literal::False,
        };

        Ok(Object::Literal(res))
    }
}

impl Interpreter {
    fn err_op(&self, msg: &str, op: &Token) -> ResultMSG<Object> {
        Err(Error::Runtime(
            op.line,
            msg.to_string(),
            format!("{:?}", op.lexeme),
        ))
    }

    fn err_near(&self, msg: &str, op: &Token, near: String) -> ResultMSG<Object> {
        Err(Error::Runtime(op.line, msg.to_string(), near))
    }
}
