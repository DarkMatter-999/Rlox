use core::fmt;
use std::error;

pub type ResultMSG<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Parser(u32, String, String),
    Runtime(u32, String, String),
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
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Parser(_, _, _) => "parse error",
            Error::Runtime(_, _, _) => "runtime error",
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
