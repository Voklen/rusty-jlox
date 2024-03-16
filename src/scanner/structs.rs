use phf::phf_map;

#[derive(Debug, PartialEq)]
pub struct Token {
	pub token_type: TokenType,
	pub lexeme: String,
	pub line: usize,
	// literal
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
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
	Identifier(String),
	String,
	Number(f64),

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

pub const KEYWORDS_MAP: phf::Map<&str, TokenType> = phf_map! {
	"and" => TokenType::And,
	"class" => TokenType::Class,
	"else" => TokenType::Else,
	"false" => TokenType::False,
	"fun" => TokenType::Fun,
	"for" => TokenType::For,
	"if" => TokenType::If,
	"nil" => TokenType::Nil,
	"or" => TokenType::Or,
	"print" => TokenType::Print,
	"return" => TokenType::Return,
	"super" => TokenType::Super,
	"this" => TokenType::This,
	"true" => TokenType::True,
	"var" => TokenType::Var,
	"while" => TokenType::While,
};
