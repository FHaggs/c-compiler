use super::token::Token;
use phf::phf_map;

pub static KEYWORDS: phf::Map<&'static str, Token> = phf_map! {
    "int" => Token::Int,
    "void" => Token::Void,
    "return" => Token::Return,
};

pub fn tokenize(chars: impl Iterator<Item = char>) -> Vec<Token> {
    let mut chars = chars.peekable();
    let mut tokens: Vec<Token> = Vec::new();

    while let Some(c) = chars.peek() {
        match c {
            '\n' | ' ' | '\t' => {
                // Do nothing
                chars.next();
            }
            '0'..'9' => {
                // Read number literal
                tokens.push(read_number_literal(&mut chars))
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                // Lex an identifier or keyword
                tokens.push(read_identifier_or_keyword(&mut chars));
            }

            '(' => {
                chars.next();
                tokens.push(Token::OpenParenthesis);
            }
            ')' => {
                chars.next();
                tokens.push(Token::ClosedParenthesis);
            }
            '{' => {
                chars.next();
                tokens.push(Token::OpenBraces);
            }
            '}' => {
                chars.next();
                tokens.push(Token::ClosedBraces);
            }
            ';' => {
                chars.next();
                tokens.push(Token::Semicolon);
            }

            _ => {
                panic!("Unexpected character: {}", c);
            }
        }
    }
    tokens.push(Token::EOF);
    tokens
}

fn read_number_literal(chars: &mut std::iter::Peekable<impl Iterator<Item = char>>) -> Token {
    // todo add float support
    let mut num = String::new();
    while let Some(ch) = chars.peek() {
        if ch.is_ascii_digit() {
            num.push(*ch);
            chars.next();
        } else {
            break;
        }
    }
    Token::IntLiteral(num.parse().unwrap())
}
fn read_identifier_or_keyword(
    chars: &mut std::iter::Peekable<impl Iterator<Item = char>>,
) -> Token {
    // todo add float support
    let mut ident = String::new();
    while let Some(&ch) = chars.peek() {
        if ch.is_alphanumeric() || ch == '_' {
            ident.push(ch);
            chars.next();
        } else {
            break;
        }
    }
    lookup_identifier(&ident)
}

fn lookup_identifier(s: &str) -> Token {
    // Get keyword or identifier
    KEYWORDS
        .get(s)
        .cloned()
        .unwrap_or(Token::Identifier(s.to_string()))
}
