use crate::scanner::structs::Token;

enum Expr {
	Literal,
	Unary,
	Binary,
	Grouping,
}

struct Binary {
	left: Expr,
	operator: Token,
	right: Expr,
}
