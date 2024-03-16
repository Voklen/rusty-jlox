use crate::error;
use anyhow::Result;
use std::{iter::Peekable, str::Chars};
use structs::*;

mod structs;
mod test;

pub fn scanner(chars: Chars) -> Result<Vec<Token>> {
	let mut tokens = vec![];
	let mut peekable = chars.peekable();
	let mut line = 1;
	loop {
		let token_result = scan_token(&mut peekable, line);
		if let Some((token, extra_lines)) = token_result {
			match token.token_type {
				TokenType::EOF => break,
				TokenType::Newline => line += 1,
				_ => tokens.push(token),
			}
			line += extra_lines;
		}
	}
	Ok(tokens)
}

fn scan_token(chars: &mut Peekable<Chars>, line: usize) -> Option<(Token, usize)> {
	let token = |token_type, lexeme| add_token(token_type, lexeme, line);
	let eq_token = |base_type, eq_type, base_lexeme, chars| {
		add_eq_token(base_type, eq_type, base_lexeme, chars, line)
	};

	let next_char = chars.next();
	let char = match next_char {
		Some(char) => char,
		None => return Some((token(EOF, ""), 0)),
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
		// Newline
		'\n' => token(Newline, "\n"),
		// Literals
		'"' => return string(chars, line),
		'0' => number(chars, line, '0'),
		'1' => number(chars, line, '1'),
		'2' => number(chars, line, '2'),
		'3' => number(chars, line, '3'),
		'4' => number(chars, line, '4'),
		'5' => number(chars, line, '5'),
		'6' => number(chars, line, '6'),
		'7' => number(chars, line, '7'),
		'8' => number(chars, line, '8'),
		'9' => number(chars, line, '9'),
		unexpected => {
			if unexpected.is_alphabetic() {
				identifier(chars, line, unexpected)
			} else {
				error!("Unexpected character: {unexpected} on line {line}");
				return None;
			}
		}
	};
	return Some((token, 0));
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

fn string(chars: &mut Peekable<Chars>, line: usize) -> Option<(Token, usize)> {
	let mut extra_lines = 0;
	let mut string_chars = vec![];
	while let Some(char) = chars.next() {
		match char {
			'\n' => {
				extra_lines += 1;
				string_chars.push(char)
			}
			'"' => break,
			char => string_chars.push(char),
		}
	}
	let string: String = string_chars.into_iter().collect();
	let token = add_token(TokenType::String, &string, line);
	Some((token, extra_lines))
}

fn number(chars: &mut Peekable<Chars<'_>>, line: usize, initial_char: char) -> Token {
	let mut number_chars = vec![initial_char];
	while let Some(char) = chars.peek() {
		if char.is_ascii_digit() || char == &'.' {
			number_chars.push(chars.next().unwrap()); // We've just peeked Some() so we know it's Some()
		} else {
			break;
		}
	}
	let string: String = number_chars.into_iter().collect();
	let as_float: f64 = string.parse().unwrap(); // We just checked it was a valid number
	let token = add_token(TokenType::Number(as_float), &string, line);
	token
}

fn identifier(chars: &mut Peekable<Chars<'_>>, line: usize, initial_char: char) -> Token {
	let mut number_chars = vec![initial_char];
	while let Some(char) = chars.peek() {
		if !char.is_alphanumeric() {
			break;
		}
		number_chars.push(chars.next().unwrap()); // We've just peeked Some() so we know it's Some()
	}
	let string: String = number_chars.into_iter().collect();
	let token_type = match KEYWORDS_MAP.get(string.as_str()) {
		Some(token_type) => (*token_type).clone(),
		None => TokenType::Identifier(string.clone()),
	};
	let token = add_token(token_type, &string, line);
	token
}
