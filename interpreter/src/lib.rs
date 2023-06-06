use std::{
    fs::File,
    io::{self, Read, Write},
};

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

    println!("{}", code);
}

pub fn run_prompt() -> io::Result<()> {
    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut line = String::new();
        stdin.read_line(&mut line)?;
        if line.is_empty() || line == "\n" {
            break;
        }
        run(&line);
    }
    Ok(())
}

fn run(code: &str) {
    todo!();
}

#[cfg(test)]
pub mod test {
    #[test]
    fn test_open() {
        assert_eq!(1, 1)
    }
}
