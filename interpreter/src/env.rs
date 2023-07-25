use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    error::{Error, ResultMSG},
    object::Object,
};

pub struct Env {
    parent: Option<Rc<Env>>,
    vals: RefCell<HashMap<String, Object>>,
}

impl Env {
    pub fn new(parent: Option<Rc<Env>>) -> Rc<Env> {
        Rc::new(Env {
            parent,
            vals: RefCell::new(HashMap::new()),
        })
    }

    pub fn with_parent(parent: Rc<Env>) -> Rc<Env> {
        Rc::new(Env {
            vals: RefCell::new(HashMap::new()),
            parent: Some(parent),
        })
    }

    pub fn with_globals(env: Rc<Env>) -> Rc<Env> {
        match env.parent {
            None => Env::with_parent(env.clone()),
            Some(ref e) => Env::with_globals(e.clone()),
        }
    }

    pub fn define(&self, name: &str, val: Object) -> ResultMSG<()> {
        let mut vals = self.vals.borrow_mut();

        if vals.contains_key(name) {
            return Err(Error::Runtime(
                0,
                format!("variable `{}` already defined", name),
                "".to_string(),
            ));
        }
        let _ = vals.insert(name.to_owned(), val);
        Ok(())
    }

    pub fn assign(&self, name: &str, val: Object) -> ResultMSG<Object> {
        let mut vals = self.vals.borrow_mut();

        if !vals.contains_key(name) {
            if let Some(ref parent) = self.parent {
                return parent.assign(name, val);
            }

            return Err(Error::Runtime(
                0,
                format!("variable `{}` is undefined", name),
                "".to_string(),
            ));
        }

        let _ = vals.insert(name.to_owned(), val.clone());
        Ok(val)
    }

    pub fn get(&self, name: &str) -> ResultMSG<Object> {
        let vals = self.vals.borrow();

        if !vals.contains_key(name) {
            if let Some(ref parent) = self.parent {
                return parent.get(name);
            }

            return Err(Error::Runtime(
                0,
                format!("variable `{}` is undefined", name),
                "".to_string(),
            ));
        }

        Ok(vals.get(name).cloned().unwrap())
    }
}
