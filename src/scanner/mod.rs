use crate::error;
use anyhow::Result;
use std::{iter::Peekable, str::Chars};

mod test;

#[derive(Debug, PartialEq)]
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

	// Newline
	Newline,

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
	let mut line = 1;
	loop {
		let token_result = scan_token(&mut peekable, line);
		if let Some(token) = token_result {
			match token.token_type {
				TokenType::EOF => break,
				TokenType::Newline => line += 1,
				_ => tokens.push(token),
			}
		}
	}
	Ok(tokens)
}

fn scan_token(chars: &mut Peekable<Chars>, line: usize) -> Option<Token> {
	let token = |token_type, lexeme| add_token(token_type, lexeme, line);
	let eq_token = |base_type, eq_type, base_lexeme, chars| {
		add_eq_token(base_type, eq_type, base_lexeme, chars, line)
	};

	let next_char = chars.next();
	let char = match next_char {
		Some(char) => char,
		None => return Some(token(EOF, "")),
	};
	let should_skip = match char {
		' ' => true,
		'\r' => true,
		'\t' => true,
		_ => false,
	};
	if should_skip {
		return None;
	}
	use TokenType::*;
	let token = match char {
		// Single-character tokens
		'(' => token(LeftParen, "("),
		')' => token(RightParen, ")"),
		'{' => token(LeftBrace, "{"),
		'}' => token(RightBrace, "}"),
		',' => token(Comma, ","),
		'.' => token(Dot, "."),
		'-' => token(Minus, "-"),
		'+' => token(Plus, "+"),
		';' => token(Semicolon, ";"),
		'*' => token(Star, "*"),
		// One or two character tokens
		'!' => eq_token(Bang, BangEqual, "!", chars),
		'=' => eq_token(Equal, EqualEqual, "=", chars),
		'<' => eq_token(Less, LessEqual, "<", chars),
		'>' => eq_token(Greater, GreaterEqual, ">", chars),
		'/' => slash(chars, line),
		'\n' => token(Newline, "\n"),
		'"' => string(chars, line)?,
		unexpected => {
			error!("Unexpected character: {unexpected} on line {line}");
			return None;
		}
	};
	return Some(token);
}

fn add_token(token_type: TokenType, lexeme: &str, line: usize) -> Token {
	return Token {
		token_type,
		lexeme: lexeme.to_string(),
		line,
	};
}

fn add_eq_token(
	base_type: TokenType,
	eq_type: TokenType,
	base_lexeme: &str,
	chars: &mut Peekable<Chars>,
	line: usize,
) -> Token {
	if chars.peek() == Some(&'=') {
		chars.next();
		add_token(eq_type, &format!("{base_lexeme}="), line)
	} else {
		add_token(base_type, base_lexeme, line)
	}
}

fn slash(chars: &mut Peekable<Chars>, line: usize) -> Token {
	if chars.peek() == Some(&'/') {
		while !reached_newline(chars) {}
		return add_token(TokenType::Newline, "\n", line);
	};
	let token = add_token(TokenType::Slash, "/", line);
	token
}

fn reached_newline(chars: &mut Peekable<Chars>) -> bool {
	let next_char = chars.next();
	next_char == None || next_char == Some('\n')
}

fn string(chars: &mut Peekable<Chars>, line: usize) -> Option<Token> {
	let mut extra_lines = 0;
	let mut string_chars = vec![];
	while let Some(char) = chars.next() {
		match char {
			'\n' => extra_lines += 1,
			'"' => break,
			char => string_chars.push(char),
		}
	}
	let string: String = string_chars.into_iter().collect();
	let token = add_token(TokenType::String, &string, line);
	Some(token)
}
