use core::fmt;
use std::error;

use crate::object::Object;

pub type ResultMSG<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Parser(u32, String, String),
    Runtime(u32, String, String),
    Break(u32),
    Return(u64, Object),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Parser(line, msg, near) => write!(
                f,
                "Parse Error at [line: {}] {} : near {} ",
                line, msg, &near
            ),
            Error::Runtime(line, msg, near) => write!(
                f,
                "Runtime Error at [line: {}] {} : near {} ",
                line, msg, &near
            ),
            Error::Break(ref line) => write!(
                f,
                "Runtime Error [line {}] unexpected break statement",
                line
            ),
            Error::Return(ref line, _) => write!(
                f,
                "Runtime Error [line {}] unexpected return statement",
                line
            ),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Parser(_, _, _) => "parse error",
            Error::Runtime(_, _, _) => "runtime error",
            Error::Break(_) => "break error",
            Error::Return(_, _) => "return error",
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            _ => None,
        }
    }
}

impl Error {
    pub fn boxed(self) -> Box<Error> {
        Box::new(self)
    }
}
