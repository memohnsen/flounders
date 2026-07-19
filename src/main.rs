use std::{env, fs, process::ExitCode};

use codecrafters_interpreter::scanner::scanner;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
    }

    let command = &args[1];
    let filename = &args[2];

    let mut exit_code = 0;

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            if !file_contents.is_empty() {
                exit_code = scanner(file_contents);
                println!("EOF  null");
            }

            ExitCode::from(exit_code)
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            ExitCode::FAILURE
        }
    }
}
