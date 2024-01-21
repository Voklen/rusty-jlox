use std::str::Chars;

use anyhow::Result;

use crate::error;

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
    // literal
}

#[derive(Debug)]
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

pub fn scanner(string: Chars) -> Result<impl Iterator<Item = Token> + '_> {
    let res = string.filter_map(scan_token);
    Ok(res)
}

fn scan_token(char: char) -> Option<Token> {
    use TokenType::*;
    let token = match char {
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
