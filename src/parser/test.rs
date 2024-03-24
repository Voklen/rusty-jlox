#[cfg(test)]
mod tests {
	use crate::parser::structs::*;
	use crate::scanner::structs::*;

	#[test]
	fn basic_test() {
		use Expr::*;
		let expr = Binary {
			left: Box::new(Literal(LiteralStruct::Number(1.0))),
			operator: Token {
				token_type: TokenType::Minus,
				lexeme: "-".to_string(),
				line: 1,
			},
			right: Box::new(Literal(LiteralStruct::Number(2.0))),
		};
		assert_eq!(expr.to_string(), "(- 1 2)");
	}

	#[test]
	fn ch_5_4_test() {
		use Expr::*;
		let expr = Binary {
			left: Box::new(Unary {
				operator: Token {
					token_type: TokenType::Minus,
					lexeme: "-".to_owned(),
					line: 1,
				},
				right: Box::new(Literal(LiteralStruct::Number(123.0))),
			}),
			operator: Token {
				token_type: TokenType::Star,
				lexeme: "*".to_owned(),
				line: 1,
			},
			right: Box::new(Grouping(Box::new(Literal(LiteralStruct::Number(45.67))))),
		};
		assert_eq!(expr.to_string(), "(* (- 123) (group 45.67))");
	}
}
