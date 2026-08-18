use codecrafters_interpreter::scanner::Scanner;
use std::{env, fs, process::ExitCode};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: flounders tokenize <filename>");
    }

    // Crash early if these aren't Some since the interpreter is useless without them
    #[allow(clippy::unwrap_used)]
    let command = &args.get(1).unwrap();
    #[allow(clippy::unwrap_used)]
    let filename = &args.get(2).unwrap();

    if command.as_str() == "tokenize" {
        let mut scanner = Scanner::default();

        let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
            eprintln!("Failed to read file {filename}");
            String::new()
        });

        if !file_contents.is_empty() {
            scanner.scan(&file_contents);
        }
        println!("EOF  null");

        if scanner.invalid_char {
            ExitCode::from(65)
        } else {
            ExitCode::SUCCESS
        }
    } else {
        eprintln!("Unknown command: {command}");
        ExitCode::FAILURE
    }
}
