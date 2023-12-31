mod error;
mod scanner;
mod token;
mod token_type;

use std::{
    env::args,
    fs::read_to_string,
    io::{self, stdin, stdout, BufRead, Write},
    process::exit,
};

use error::LoxError;
use scanner::Scanner;

fn main() {
    let args: Vec<String> = args().collect();

    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]).expect("Error running file"),
        _ => {
            println!("Usage: lax-ast [file]");
            exit(64);
        }
    }
}

fn run_file(path: &str) -> io::Result<()> {
    let buf = read_to_string(path)?;

    if run(buf).is_err() {
        exit(65);
    }

    Ok(())
}

fn run_prompt() {
    let stdin = stdin();

    print!("> ");
    stdout().flush().unwrap();

    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }

            let _ = run(line);
        } else {
            break;
        }

        print!("> ");
        stdout().flush().unwrap();
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
