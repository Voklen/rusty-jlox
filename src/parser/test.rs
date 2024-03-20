#[cfg(test)]
mod tests {
	use crate::parser::structs::*;
	use crate::scanner::structs::*;

	#[test]
	fn ch_5_4_test() {
		let expr = Expr::Binary {
			left: Box::new(Expr::Literal(Literal::Number(1))),
			operator: Token {
				token_type: TokenType::Minus,
				lexeme: "-".to_string(),
				line: 1,
			},
			right: Box::new(Expr::Literal(Literal::Number(2))),
		};
		assert_eq!(expr.to_string(), "(- 1 2)");
	}
}
