use std::{ops::Deref, rc::Rc};

use crate::{
    env::Env,
    error::{Error, ResultMSG},
    expr::{Expr, Visitor as ExprVisitor},
    object::Object,
    stmt::{Stmt, Visitor as StmtVisitor},
    token::{Literal, Token, TokenType},
};

pub struct Interpreter {
    env: Rc<Env>,
    repl: bool,
}

impl Interpreter {
    pub fn new(repl: bool) -> Self {
        Interpreter {
            env: Env::new(None),
            repl,
        }
    }

    pub fn interpret(&mut self, s: &Stmt) -> ResultMSG<()> {
        s.accept(self)
    }

    fn scoped(&self) -> Self {
        Interpreter {
            env: Env::new(Some(self.env.clone())),
            repl: self.repl,
        }
    }

    pub fn evaluate(&mut self, expr: &Expr) -> ResultMSG<Object> {
        self.visit_grouping(expr, expr)
    }
}

impl ExprVisitor<ResultMSG<Object>> for Interpreter {
    fn visit_expr(&mut self, expr: &Expr) -> ResultMSG<Object> {
        match *expr {
            Expr::Identifier(ref name) => self.visit_identifier(expr, name),
            Expr::Unary(ref op, ref rhs) => self.visit_unary(expr, op, rhs),
            Expr::Binary(ref lhs, ref op, ref rhs) => self.visit_binary(expr, lhs, op, rhs),
            Expr::Literal(ref lit) => self.visit_literal(expr, lit),
            Expr::Grouping(ref inside) => self.visit_grouping(expr, inside),
            Expr::Assignment(ref id, ref rhs) => self.visit_assignment(expr, id, rhs),
        }
    }

    fn visit_literal(&mut self, expr: &Expr, lit: &Token) -> ResultMSG<Object> {
        Ok(Object::Literal(lit.literal.clone()))
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

    fn visit_identifier(&mut self, expr: &Expr, n: &String) -> ResultMSG<Object> {
        self.env.get(&n).map(|lit| lit.clone())
    }

    fn visit_assignment(&mut self, expr: &Expr, n: &String, rhs: &Box<Expr>) -> ResultMSG<Object> {
        let val = self.evaluate(rhs)?;
        self.env.assign(&n, val).map(|lit| lit.clone())
    }
}

impl StmtVisitor<ResultMSG<()>> for Interpreter {
    fn visit_stmt(&mut self, s: &Stmt) -> ResultMSG<()> {
        match *s {
            Stmt::Empty => Ok(()),
            Stmt::Print(ref e) => self.visit_print_stmt(e),
            Stmt::Expression(ref e) => self.visit_expression_stmt(e),
            Stmt::Block(ref ss) => self.visit_block_stmt(ss),
            Stmt::Declaration(ref n, ref e) => self.visit_declaration_stmt(n, e.as_ref()),
            Stmt::If(ref c, ref t, ref e) => {
                self.visit_if(c, t.as_ref(), e.as_ref().map(|x| x.deref()))
            }
            Stmt::While(ref e, ref b) => self.visit_while(e, b.deref()),
            Stmt::Break(l) => self.visit_break(l),
        }
    }

    fn visit_expression_stmt(&mut self, e: &Expr) -> ResultMSG<()> {
        if self.repl {
            self.visit_print_stmt(e)
        } else {
            e.accept(self).map(|_| ())
        }
    }

    fn visit_print_stmt(&mut self, expression: &Expr) -> ResultMSG<()> {
        println!("{:?}", expression.accept(self)?);
        Ok(())
    }

    fn visit_declaration_stmt(&mut self, name: &String, init: Option<&Expr>) -> ResultMSG<()> {
        let val: Object =
            init.map_or_else(|| Ok(Object::Literal(Literal::None)), |e| e.accept(self))?;
        self.env.define(&name, val)
    }

    fn visit_block_stmt(&mut self, statements: &Vec<Stmt>) -> ResultMSG<()> {
        let mut scope: Self = self.scoped();
        for stmt in statements {
            stmt.accept(&mut scope)?;
        }
        Ok(())
    }

    fn visit_if(
        &mut self,
        expr: &Expr,
        then_stmt: &Stmt,
        else_stmt: Option<&Stmt>,
    ) -> ResultMSG<()> {
        let cond = expr.accept(self)?;

        if cond.is_truthy() {
            return then_stmt.accept(self);
        }

        if else_stmt.is_none() {
            Ok(())
        } else {
            else_stmt.unwrap().deref().accept(self)
        }
    }

    fn visit_while(&mut self, expr: &Expr, body: &Stmt) -> ResultMSG<()> {
        while self.evaluate(expr)?.is_truthy() {
            match body.accept(self) {
                Err(Error::Break(_)) => break,
                Err(e) => return Err(e),
                Ok(_) => (),
            };
        }

        Ok(())
    }

    fn visit_break(&mut self, line: u32) -> ResultMSG<()> {
        Err(Error::Break(line))
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
