use std::{
    fs::File,
    io::{self, stderr, BufRead, BufReader, Read, Write},
};

use error::ResultMSG;
use scanner::{Scanner, StmtIterator};

use crate::{interpreter::Interpreter, parser::Parser};

mod env;
mod error;
mod expr;
mod interpreter;
mod object;
mod parser;
mod scanner;
mod stmt;
mod token;

pub fn run_file(path: String) {
    let mut file = match File::open(path.clone()) {
        Err(e) => panic!("Could not open file {}\n{}", path, e),
        Ok(file) => file,
    };

    let mut code = String::new();
    match file.read_to_string(&mut code) {
        Err(e) => panic!("Error reading file {}\n{}", path, e),
        Ok(_) => (),
    }

    let mut i = Interpreter::new(false);

    match run(&code, &mut i) {
        Ok(t) => {}
        Err(e) => {
            return;
        }
    }
}

pub fn run_prompt() -> io::Result<()> {
    let stdin = io::stdin();

    let mut i = Interpreter::new(true);
    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut line = String::new();
        stdin.read_line(&mut line)?;
        if line.is_empty() || line == "\n" {
            break;
        }
        match run(&line, &mut i) {
            Ok(t) => {}
            Err(e) => {
                std::process::exit(65);
            }
        }
    }
    Ok(())
}

fn run(code: &str, interpreter: &mut Interpreter) -> ResultMSG<()> {
    let mut scanner = Scanner::new(code.to_string());
    let tokens = scanner.scan_tokens();

    for token in &tokens {
        println!("{:?}", token);
    }

    for res in scanner.statements() {
        match res {
            Err(e) => {
                writeln!(&mut stderr(), "{}", e);
                break;
            }
            Ok(stmt) => interpreter.interpret(&stmt)?,
        }
    }

    /*
     * let mut parser = Parser::new(tokens);
    let stmt = parser.parse();

    println!("{:#?}", stmt);

    if let Ok(stmt) = stmt {
        let out = interpreter.interpret(&stmt);

        println!("{:?}", out);
    }

    */
    return Ok(());
}

#[cfg(test)]
pub mod test {
    #[test]
    fn test_open() {
        assert_eq!(1, 1)
    }
}
