use interpreter::{run_file, run_prompt};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    dbg!(&args);

    if args.len() > 2 {
        println!("Usage: rlox <script>")
    } else if args.len() == 2 {
        run_file(args[1].clone());
    } else {
        match run_prompt() {
            Err(e) => panic!("Run prompt {}", e),
            Ok(_) => (),
        }
    }
}
