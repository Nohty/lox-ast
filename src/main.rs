mod error;
mod scanner;
mod token;
mod token_type;

use std::{
    env::args,
    fs::read_to_string,
    io::{self, stdin, BufRead},
    process::exit,
};

use error::LoxError;
use scanner::Scanner;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() > 2 {
        println!("Usage: lax-ast [file]");
        exit(64)
    } else if args.len() == 2 {
        run_file(&args[0]).expect("Error running file")
    } else {
        run_prompt();
    }
}

fn run_file(path: &str) -> io::Result<()> {
    let buf = read_to_string(path)?;

    match run(buf) {
        Ok(_) => {}
        Err(e) => {
            e.report();
            exit(65);
        }
    }

    Ok(())
}

fn run_prompt() {
    let stdin = stdin();

    print!("> ");

    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }

            match run(line) {
                Ok(_) => {}
                Err(e) => {
                    e.report();
                    exit(65);
                }
            }
        } else {
            break;
        }
    }
}

fn run(source: String) -> Result<(), LoxError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}
