use std::fmt;

use crate::scanner::structs::Token;

pub enum Expr {
	Literal(LiteralStruct),
	Unary {
		operator: Token,
		right: Box<Expr>,
	},
	Binary {
		left: Box<Expr>,
		operator: Token,
		right: Box<Expr>,
	},
	Grouping(Box<Expr>),
}

pub enum LiteralStruct {
	Number(f64),
}

impl fmt::Display for Expr {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Expr::Literal(expr) => write!(f, "{}", expr),
			Expr::Unary { operator, right } => write!(f, "({} {})", operator.lexeme, right),
			Expr::Binary {
				left,
				operator,
				right,
			} => {
				write!(f, "({} {} {})", operator.lexeme, left, right)
			}
			Expr::Grouping(expr) => write!(f, "(group {})", expr),
		}
	}
}

impl fmt::Display for LiteralStruct {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Number(num) => write!(f, "{}", num),
		}
	}
}
