use std::{iter::Peekable, str::Chars};

use anyhow::Result;

use crate::error;

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
    // literal
}

#[derive(Debug, PartialEq)]
enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

pub fn scanner(chars: Chars) -> Result<Vec<Token>> {
    let mut tokens = vec![];
    let mut peekable = chars.peekable();
    loop {
        let token = scan_token(&mut peekable);
        match token {
            Some(token) => {
                if token.token_type == TokenType::EOF {
                    break;
                }
                tokens.push(token)
            }
            None => {}
        }
    }
    Ok(tokens)
}

fn scan_token(chars: &mut Peekable<Chars>) -> Option<Token> {
    let next_char = chars.next();
    let char = match next_char {
        Some(char) => char,
        None => return Some(add_token(EOF, "")),
    };
    use TokenType::*;
    let token = match char {
        // Single-character tokens
        '(' => add_token(LeftParen, "("),
        ')' => add_token(RightParen, ")"),
        '{' => add_token(LeftBrace, "{"),
        '}' => add_token(RightBrace, "}"),
        ',' => add_token(Comma, ","),
        '.' => add_token(Dot, "."),
        '-' => add_token(Minus, "-"),
        '+' => add_token(Plus, "+"),
        ';' => add_token(Semicolon, ";"),
        '*' => add_token(Star, "*"),
        // One or two character tokens
        '!' => add_eq_token(Bang, BangEqual, "!", chars),
        '=' => add_eq_token(Equal, EqualEqual, "=", chars),
        '<' => add_eq_token(Less, LessEqual, "<", chars),
        '>' => add_eq_token(Greater, GreaterEqual, ">", chars),
        '/' => slash(chars)?,
        unexpected => {
            error!("Unexpected character: {unexpected}");
            return None;
        }
    };
    return Some(token);
}

fn add_token(token_type: TokenType, lexeme: &str) -> Token {
    return Token {
        token_type,
        lexeme: lexeme.to_string(),
        line: 1,
    };
}

fn add_eq_token(
    base_type: TokenType,
    eq_type: TokenType,
    base_lexeme: &str,
    chars: &mut Peekable<Chars>,
) -> Token {
    if chars.peek() == Some(&'=') {
        add_token(eq_type, &format!("={base_lexeme}"))
    } else {
        add_token(base_type, base_lexeme)
    }
}

fn slash(chars: &mut Peekable<Chars>) -> Option<Token> {
    if chars.peek() == Some(&'/') {
        while !reached_newline(chars) {}
        return None;
    };
    let token = add_token(TokenType::Slash, "/");
    Some(token)
}

fn reached_newline(chars: &mut Peekable<Chars>) -> bool {
    let next_char = chars.next();
    next_char == None || next_char == Some('\n')
}
