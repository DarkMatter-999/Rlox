use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read, Write},
};

use scanner::Scanner;

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

    let reader = BufReader::new(file);

    let mut i = Interpreter::new(false);

    /*    let mut code = String::new();
        match file.(&mut code) {
            Err(e) => panic!("Error reading file {}\n{}", path, e),
            Ok(_) => (),
        }


        let mut i = Interpreter::new(false);

        run(&code, &mut i);
    */
    for line in reader.lines() {
        match line {
            Err(e) => panic!("Error reading file {}\n{}", path, e),
            Ok(code) => {
                match run(&code, &mut i) {
                    Ok(t) => {}
                    Err(e) => {
                        std::process::exit(65);
                    }
                };
            }
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

fn run(code: &str, interpreter: &mut Interpreter) -> Result<bool, ()> {
    let mut scanner = Scanner::new(code.to_string());
    let tokens = scanner.scan_tokens();

    for token in &tokens {
        println!("{:?}", token);
    }

    let mut parser = Parser::new(tokens);
    let stmt = parser.parse();

    println!("{:#?}", stmt);

    if let Ok(stmt) = stmt {
        let out = interpreter.interpret(&stmt);

        println!("{:?}", out);
    }

    return Ok(true);
}

#[cfg(test)]
pub mod test {
    #[test]
    fn test_open() {
        assert_eq!(1, 1)
    }
}
