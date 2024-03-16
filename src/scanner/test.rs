#[cfg(test)]
mod tests {
	use crate::scanner::*;

	#[test]
	fn ch_4_6_test() {
		let string = "\
// this is a comment
(( )){} // grouping stuff
!*+-/=<> <= == // operators
		";
		let actual = scanner(string.chars()).unwrap();
		use TokenType::*;
		let expected = [
			Token {
				token_type: LeftParen,
				lexeme: "(".to_string(),
				line: 2,
			},
			Token {
				token_type: LeftParen,
				lexeme: "(".to_string(),
				line: 2,
			},
			Token {
				token_type: RightParen,
				lexeme: ")".to_string(),
				line: 2,
			},
			Token {
				token_type: RightParen,
				lexeme: ")".to_string(),
				line: 2,
			},
			Token {
				token_type: LeftBrace,
				lexeme: "{".to_string(),
				line: 2,
			},
			Token {
				token_type: RightBrace,
				lexeme: "}".to_string(),
				line: 2,
			},
			Token {
				token_type: Bang,
				lexeme: "!".to_string(),
				line: 3,
			},
			Token {
				token_type: Star,
				lexeme: "*".to_string(),
				line: 3,
			},
			Token {
				token_type: Plus,
				lexeme: "+".to_string(),
				line: 3,
			},
			Token {
				token_type: Minus,
				lexeme: "-".to_string(),
				line: 3,
			},
			Token {
				token_type: Slash,
				lexeme: "/".to_string(),
				line: 3,
			},
			Token {
				token_type: Equal,
				lexeme: "=".to_string(),
				line: 3,
			},
			Token {
				token_type: Less,
				lexeme: "<".to_string(),
				line: 3,
			},
			Token {
				token_type: Greater,
				lexeme: ">".to_string(),
				line: 3,
			},
			Token {
				token_type: LessEqual,
				lexeme: "<=".to_string(),
				line: 3,
			},
			Token {
				token_type: EqualEqual,
				lexeme: "==".to_string(),
				line: 3,
			},
		];
		assert_eq!(actual, expected);
	}

	#[test]
	fn string_test() {
		let string = "\
		\"string\"
		*
		\"other string (with spaces)\"
		";
		let actual = scanner(string.chars()).unwrap();
		use TokenType::*;
		let expected = [
			Token {
				token_type: String,
				lexeme: "string".to_string(),
				line: 1,
			},
			Token {
				token_type: Star,
				lexeme: "*".to_string(),
				line: 2,
			},
			Token {
				token_type: String,
				lexeme: "other string (with spaces)".to_string(),
				line: 3,
			},
		];
		assert_eq!(actual, expected);
	}

	#[test]
	fn multiline_string_test() {
		let string = "\
		\"this string
starts on one line
and ends on another\"
		*
		\"other string (with spaces)\"
		";
		let actual = scanner(string.chars()).unwrap();
		use TokenType::*;
		let expected = [
			Token {
				token_type: String,
				lexeme: "this string\nstarts on one line\nand ends on another".to_string(),
				line: 1,
			},
			Token {
				token_type: Star,
				lexeme: "*".to_string(),
				line: 4,
			},
			Token {
				token_type: String,
				lexeme: "other string (with spaces)".to_string(),
				line: 5,
			},
		];
		assert_eq!(actual, expected);
	}
}
