use codecrafters_interpreter::scanner::Scanner;
use std::{env, fs, process::ExitCode};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let mut scanner = Scanner::default();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            if !file_contents.is_empty() {
                scanner.scan(file_contents);
                println!("EOF  null");
            } else {
                println!("EOF  null");
            }

            if scanner.invalid_char.is_none() {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(65)
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            ExitCode::FAILURE
        }
    }
}
