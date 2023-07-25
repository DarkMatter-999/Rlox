use std::{borrow::Borrow, rc::Rc};

use crate::{
    env::Env,
    error::{Error, ResultMSG},
    interpreter::Interpreter,
    object::Object,
    stmt::Stmt,
    token::Literal,
};

pub trait Callable {
    fn call(&self, int: &mut Interpreter, args: &[Object]) -> ResultMSG<Object>;
    fn arity(&self) -> usize;
}

pub struct LoxFunction {
    env: Rc<Env>,
    params: Vec<String>,
    body: Rc<Stmt>,
}

impl LoxFunction {
    pub fn new(env: Rc<Env>, params: &[String], body: Rc<Stmt>) -> Rc<LoxFunction> {
        Rc::new(LoxFunction {
            env,
            params: params.to_vec(),
            body,
        })
    }
}

impl Callable for LoxFunction {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(&self, int: &mut Interpreter, args: &[Object]) -> ResultMSG<Object> {
        let env = Env::with_parent(self.env.clone());
        let params = &self.params;
        let zip = params.into_iter().zip(args.into_iter());

        for (param, arg) in zip {
            env.define(param, arg.clone());
        }

        match Interpreter::with_env(env).interpret(self.body.borrow()) {
            Ok(()) => Ok(Object::Literal(Literal::None)),
            Err(Error::Return(_, res)) => Ok(res),
            Err(e) => Err(e),
        }
    }
}
