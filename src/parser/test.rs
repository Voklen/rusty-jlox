#[cfg(test)]
mod tests {
	use crate::parser::structs::*;
	use crate::scanner::structs::*;

	#[test]
	fn ch_5_4_test() {
		let binding = Binary {
			left: Expr::Literal(Literal::Number(1)),
			operator: Token {
				token_type: TokenType::Minus,
				lexeme: "-".to_string(),
				line: 1,
			},
			right: Expr::Literal(Literal::Number(2)),
		};
		let expr = Expr::Binary(Box::new(binding));
		assert_eq!(expr.to_string(), "(- 1 2)");
	}
}
