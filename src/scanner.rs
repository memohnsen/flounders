use crate::TokenType;

#[derive(Debug)]
pub struct Scanner {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: String,
    pub invalid_char: Option<bool>,
}

impl Default for Scanner {
    fn default() -> Self {
        Self {
            token_type: TokenType::LeftParen,
            lexeme: "".to_string(),
            literal: "".to_string(),
            invalid_char: None,
        }
    }
}

impl Scanner {
    pub fn scan(&mut self, contents: String) {
        let mut end_of_file = false;

        while !end_of_file {
            let lines: Vec<&str> = contents.lines().collect();

            for (current_line, line) in lines.into_iter().enumerate() {
                let current_line = current_line + 1;
                let mut chars = line.chars().peekable();

                while let Some(current) = chars.next() {
                    let handle_double_chars = match_next_char(current, chars.peek());
                    if handle_double_chars.len() > 1 {
                        chars.next();
                    }

                    if from_lexeme(&handle_double_chars).is_none() {
                        eprintln!(
                            "[line {}] Error: Unexpected character: {}",
                            current_line, current
                        );
                        self.invalid_char = Some(true);
                    } else {
                        self.token_type = from_lexeme(&handle_double_chars.to_string())
                            .unwrap_or(TokenType::Identifier);
                        self.lexeme = handle_double_chars.to_string();
                        self.literal = "null".to_string();

                        println!("{} {} {}", self.token_type, self.lexeme, self.literal);
                    };
                }
            }
            end_of_file = true;
        }
    }
}

// Look at the current char and the next char to see if they make up a lexeme together
// <= >= != ==
// if not just return back the string
pub fn match_next_char(current: char, next: Option<&char>) -> String {
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
        _ => current.to_string(),
    }
}

pub fn from_lexeme(lexeme: &str) -> Option<TokenType> {
    match lexeme {
        "(" => Some(TokenType::LeftParen),
        ")" => Some(TokenType::RightParen),
        "{" => Some(TokenType::LeftBrace),
        "}" => Some(TokenType::RightBrace),
        "," => Some(TokenType::Comma),
        "." => Some(TokenType::Dot),
        "-" => Some(TokenType::Minus),
        "+" => Some(TokenType::Plus),
        ";" => Some(TokenType::Semicolon),
        "/" => Some(TokenType::Slash),
        "*" => Some(TokenType::Star),
        "!" => Some(TokenType::Bang),
        "!=" => Some(TokenType::BangEqual),
        "=" => Some(TokenType::Equal),
        "==" => Some(TokenType::EqualEqual),
        ">" => Some(TokenType::Greater),
        ">=" => Some(TokenType::GreaterEqual),
        "<" => Some(TokenType::Less),
        "<=" => Some(TokenType::LessEqual),
        "and" => Some(TokenType::And),
        "class" => Some(TokenType::Class),
        "else" => Some(TokenType::Else),
        "false" => Some(TokenType::False),
        "fun" => Some(TokenType::Fun),
        "for" => Some(TokenType::For),
        "if" => Some(TokenType::If),
        "nil" => Some(TokenType::Nil),
        "or" => Some(TokenType::Or),
        "print" => Some(TokenType::Print),
        "return" => Some(TokenType::Return),
        "super" => Some(TokenType::Super),
        "this" => Some(TokenType::This),
        "true" => Some(TokenType::True),
        "var" => Some(TokenType::Var),
        "while" => Some(TokenType::While),
        _ => None,
    }
}
