use crate::token::{TokenType, from_lexeme};
use std::{char, iter::Peekable, str::Chars};

#[derive(Debug, PartialEq)]
pub struct Scanner {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: String,
    pub invalid_char: bool,
    pub unterminated_string: bool,
}

impl Default for Scanner {
    fn default() -> Self {
        Self {
            token_type: TokenType::LeftParen,
            lexeme: String::new(),
            literal: String::new(),
            invalid_char: false,
            unterminated_string: false,
        }
    }
}

impl Scanner {
    fn print_error(&mut self, current_line: usize, current: char) {
        if self.unterminated_string {
            eprintln!("[line {current_line}] Error: Unterminated string.");
            self.unterminated_string = false;
        } else {
            eprintln!("[line {current_line}] Error: Unexpected character: {current}");
        }
        self.invalid_char = true;
    }

    fn lex_char(&mut self, lex: &str) {
        if lex.ends_with('"') && lex.len() > 1 {
            self.token_type = TokenType::String;
            self.literal = lex.trim_matches('"').to_string();
        } else if lex.parse::<f32>().is_ok() {
            self.token_type = TokenType::Number;
            self.literal = format_decimal(lex.parse().unwrap_or(0.0));
        } else {
            self.token_type = from_lexeme(lex).unwrap_or(TokenType::Identifier);
            self.literal = "null".to_string();
        }
        self.lexeme = lex.to_string();

        println!("{} {} {}", self.token_type, self.lexeme, self.literal);
    }

    pub fn scan(&mut self, contents: &str) {
        let mut end_of_file = false;

        while !end_of_file {
            let lines: Vec<&str> = contents.lines().collect();

            for (mut current_line, line) in lines.into_iter().enumerate() {
                current_line += 1;
                let mut chars = line.chars().peekable();

                while let Some(current) = chars.next() {
                    let lex = match_next_char(current, chars.peek());
                    if lex.len() > 1 {
                        chars.next();
                    }

                    if lex == "//" {
                        break;
                    }

                    if is_whitespace(&lex) {
                        continue;
                    }

                    if lex.parse::<u32>().is_ok() {
                        let digits = handle_numbers(&lex, &mut chars);
                        self.lex_char(&digits);
                        continue;
                    }

                    if lex == "\"" {
                        let collected = handle_quotations(&mut chars);

                        if collected.ends_with('"') && collected.len() > 1 {
                            self.lex_char(&collected);
                        } else {
                            self.unterminated_string = true;
                            self.print_error(current_line, current);
                        }
                        continue;
                    }

                    if lex.chars().all(char::is_alphabetic) || lex == "_" {
                        let collected = handle_identifiers(&lex, &mut chars);
                        self.lex_char(&collected);
                        continue;
                    }

                    if from_lexeme(&lex).is_some() {
                        self.lex_char(&lex);
                    } else {
                        self.print_error(current_line, current);
                    }
                }
            }
            end_of_file = true;
        }
    }
}

fn handle_identifiers(lex: &str, chars: &mut Peekable<Chars<'_>>) -> String {
    let mut collected = String::from(lex);
    while let Some(&c) = chars.peek() {
        if !c.is_alphanumeric() && c != '_' {
            break;
        }
        collected.push(c);
        chars.next();
    }

    collected
}

fn handle_quotations(chars: &mut Peekable<Chars<'_>>) -> String {
    let mut collected = String::from("\"");
    for c in chars.by_ref() {
        collected.push(c);
        if c == '"' {
            break;
        }
    }

    collected
}

fn handle_numbers(lex: &str, chars: &mut Peekable<Chars<'_>>) -> String {
    let mut digits = String::new();
    digits.push_str(lex);
    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() || c == '.' {
            digits.push(c);
            chars.next();
        } else {
            break;
        }
    }

    digits
}

fn is_whitespace(current: &str) -> bool {
    current == " " || current == "\n" || current == "\t" || current == "\r"
}

fn format_decimal(num: f64) -> String {
    if num.fract() == 0.0 {
        // Outputs with .0 if integer
        format!("{num:.1}")
    } else {
        // Outputs all decimals if they exist
        format!("{num}")
    }
}

// Look at the current char and the next char to see if they make up a lexeme together
// <= >= != ==
// if not just return back the string
fn match_next_char(current: char, next: Option<&char>) -> String {
    if next.is_none() {
        return current.to_string();
    }

    match current {
        '=' => match next.unwrap_or(&'=') {
            '=' => "==".to_string(),
            _ => current.to_string(),
        },
        '!' => match next.unwrap_or(&'!') {
            '=' => "!=".to_string(),
            _ => current.to_string(),
        },
        '<' => match next.unwrap_or(&'<') {
            '=' => "<=".to_string(),
            _ => current.to_string(),
        },
        '>' => match next.unwrap_or(&'>') {
            '=' => ">=".to_string(),
            _ => current.to_string(),
        },
        '/' => match next.unwrap_or(&'/') {
            '/' => "//".to_string(),
            _ => current.to_string(),
        },
        _ => current.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_next_char() {
        assert_eq!(match_next_char('=', Some(&'=')), "==");
        assert_eq!(match_next_char('=', Some(&'/')), "=");
        assert_eq!(match_next_char('!', Some(&'=')), "!=");
        assert_eq!(match_next_char('!', Some(&'/')), "!");
        assert_eq!(match_next_char('<', Some(&'=')), "<=");
        assert_eq!(match_next_char('<', Some(&'-')), "<");
        assert_eq!(match_next_char('>', Some(&'=')), ">=");
        assert_eq!(match_next_char('>', Some(&'k')), ">");
        assert_eq!(match_next_char('/', Some(&'/')), "//");
        assert_eq!(match_next_char('/', Some(&'=')), "/");
        assert_eq!(match_next_char('k', Some(&'=')), "k");
    }

    #[test]
    fn test_white_space() {
        assert!(is_whitespace(" "));
        assert!(is_whitespace("\n"));
        assert!(is_whitespace("\r"));
        assert!(is_whitespace("\t"));
        assert!(!is_whitespace("t"));
    }

    #[test]
    fn test_lexing_chars_mult_chars() {
        let mut scanner = Scanner::default();

        scanner.lex_char("=");
        assert!(!scanner.unterminated_string);
        assert!(!scanner.invalid_char);
        assert_eq!(scanner.token_type, TokenType::Equal);
        assert_eq!(scanner.literal, "null");
        assert_eq!(scanner.lexeme, "=");

        scanner.lex_char("<");
        assert!(!scanner.unterminated_string);
        assert!(!scanner.invalid_char);
        assert_eq!(scanner.token_type, TokenType::Less);
        assert_eq!(scanner.literal, "null");
        assert_eq!(scanner.lexeme, "<");

        scanner.lex_char(">");
        assert!(!scanner.unterminated_string);
        assert!(!scanner.invalid_char);
        assert_eq!(scanner.token_type, TokenType::Greater);
        assert_eq!(scanner.literal, "null");
        assert_eq!(scanner.lexeme, ">");

        scanner.lex_char("!");
        assert!(!scanner.unterminated_string);
        assert!(!scanner.invalid_char);
        assert_eq!(scanner.token_type, TokenType::Bang);
        assert_eq!(scanner.literal, "null");
        assert_eq!(scanner.lexeme, "!");

        scanner.lex_char("!=");
        assert!(!scanner.unterminated_string);
        assert!(!scanner.invalid_char);
        assert_eq!(scanner.token_type, TokenType::BangEqual);
        assert_eq!(scanner.literal, "null");
        assert_eq!(scanner.lexeme, "!=");

        scanner.lex_char(">=");
        assert!(!scanner.unterminated_string);
        assert!(!scanner.invalid_char);
        assert_eq!(scanner.token_type, TokenType::GreaterEqual);
        assert_eq!(scanner.literal, "null");
        assert_eq!(scanner.lexeme, ">=");

        scanner.lex_char("<=");
        assert!(!scanner.unterminated_string);
        assert!(!scanner.invalid_char);
        assert_eq!(scanner.token_type, TokenType::LessEqual);
        assert_eq!(scanner.literal, "null");
        assert_eq!(scanner.lexeme, "<=");

        scanner.lex_char("==");
        assert!(!scanner.unterminated_string);
        assert!(!scanner.invalid_char);
        assert_eq!(scanner.token_type, TokenType::EqualEqual);
        assert_eq!(scanner.literal, "null");
        assert_eq!(scanner.lexeme, "==");
    }

    #[test]
    fn test_lexing_strings() {
        let mut scanner = Scanner::default();

        scanner.lex_char("\"hello\"");
        assert!(!scanner.unterminated_string);
        assert!(!scanner.invalid_char);
        assert_eq!(scanner.token_type, TokenType::String);
        assert_eq!(scanner.literal, "hello");
        assert_eq!(scanner.lexeme, "\"hello\"");

        scanner.lex_char("\"hello");
        assert!(!scanner.unterminated_string);
        assert!(!scanner.invalid_char);
        assert_eq!(scanner.token_type, TokenType::Identifier);
        assert_eq!(scanner.literal, "null");
        assert_eq!(scanner.lexeme, "\"hello");

        scanner.scan("\"\"");
        assert!(!scanner.unterminated_string);
        assert!(!scanner.invalid_char);
        assert_eq!(scanner.token_type, TokenType::String);
        assert_eq!(scanner.literal, "");
        assert_eq!(scanner.lexeme, "\"\"");
    }

    #[test]
    fn identifier_edge_cases() {
        let mut scanner = Scanner::default();

        scanner.scan("foo");
        assert!(!scanner.unterminated_string);
        assert!(!scanner.invalid_char);
        assert_eq!(scanner.token_type, TokenType::Identifier);
        assert_eq!(scanner.literal, "null");
        assert_eq!(scanner.lexeme, "foo");

        scanner.scan("_foo");
        assert!(!scanner.unterminated_string);
        assert!(!scanner.invalid_char);
        assert_eq!(scanner.token_type, TokenType::Identifier);
        assert_eq!(scanner.literal, "null");
        assert_eq!(scanner.lexeme, "_foo");

        scanner.scan("foo)");
        assert!(!scanner.unterminated_string);
        assert!(!scanner.invalid_char);
        assert_eq!(scanner.token_type, TokenType::RightParen);
        assert_eq!(scanner.literal, "null");
        assert_eq!(scanner.lexeme, ")");

        scanner.scan("_123bar");
        assert!(!scanner.unterminated_string);
        assert!(!scanner.invalid_char);
        assert_eq!(scanner.token_type, TokenType::Identifier);
        assert_eq!(scanner.literal, "null");
        assert_eq!(scanner.lexeme, "_123bar");
    }

    #[test]
    fn unterminated_string() {
        let mut scanner = Scanner::default();

        scanner.scan("\"hello");
        assert!(scanner.invalid_char);

        scanner.invalid_char = false;
        scanner.scan("\"");
        assert!(scanner.invalid_char);
    }

    #[test]
    fn numbers() {
        let mut scanner = Scanner::default();

        scanner.scan("42");
        assert!(!scanner.unterminated_string);
        assert!(!scanner.invalid_char);
        assert_eq!(scanner.token_type, TokenType::Number);
        assert_eq!(scanner.literal, "42.0");
        assert_eq!(scanner.lexeme, "42");
    }

    #[test]
    fn comments() {
        let mut scanner = Scanner::default();

        scanner.scan(">// hello");
        assert!(!scanner.unterminated_string);
        assert!(!scanner.invalid_char);
        assert_eq!(scanner.token_type, TokenType::Greater);
        assert_eq!(scanner.literal, "null");
        assert_eq!(scanner.lexeme, ">");
    }
}
