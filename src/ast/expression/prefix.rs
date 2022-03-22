use crate::token::Token;

use super::Expression;

pub struct Prefix<'a> {
    pub operator: Operator,
    pub expression: Box<Expression<'a>>,
}

pub enum Operator {
    Plus,
    Minus,
    Exclamation,
}

impl Operator {
    pub fn from<'a>(token: Token<'a>) -> Option<Operator> {
        let operator = match token {
            Token::Plus => Operator::Plus,
            Token::Minus => Operator::Minus,
            Token::Exclamation => Operator::Exclamation,
            _ => return None,
        };

        Some(operator)
    }
}