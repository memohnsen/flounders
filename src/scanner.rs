use crate::TokenType;

#[derive(Debug)]
struct Scanner {
    token_type: TokenType,
    lexeme: String,
    literal: String,
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

pub fn scanner(contents: String) {
    let mut end_of_file = false;

    while !end_of_file {
        let lines: Vec<&str> = contents.lines().collect();

        for line in lines {
            for c in line.chars() {
                let scanned_line = Scanner {
                    token_type: from_lexeme(&c.to_string()).unwrap_or(TokenType::Identifier),
                    lexeme: c.to_string(),
                    literal: "null".to_string(),
                };

                println!(
                    "{} {} {}",
                    scanned_line.token_type, scanned_line.lexeme, scanned_line.literal
                );
            }
        }
        end_of_file = true;
    }
    println!("EOF  null");
}
