use std::fmt;

use crate::scanner::structs::Token;

pub enum Expr {
	Literal(Literal),
	Unary(Box<Unary>),
	Binary(Box<Binary>),
	Grouping(Box<Grouping>),
}

pub struct Unary {
	pub operator: Token,
	pub right: Expr,
}

pub struct Binary {
	pub left: Expr,
	pub operator: Token,
	pub right: Expr,
}

pub struct Grouping {
	pub expression: Expr,
}

pub enum Literal {
	Number(i64),
}

impl fmt::Display for Expr {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Expr::Literal(expr) => write!(f, "{}", expr),
			Expr::Unary(expr) => write!(f, "({} {})", expr.operator.lexeme, expr.right),
			Expr::Binary(expr) => {
				write!(f, "({} {} {})", expr.operator.lexeme, expr.left, expr.right)
			}
			Expr::Grouping(expr) => write!(f, "(group {})", expr.expression),
		}
	}
}

impl fmt::Display for Literal {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Literal::Number(num) => write!(f, "{}", num),
		}
	}
}
