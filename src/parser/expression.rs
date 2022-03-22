use crate::{ast::expression::{identifier::Identifier, Expression, prefix::{Prefix, Operator}, integer::Integer}, token::{Token, self}};

use super::Parser;

impl<'a> Parser<'a> {
    pub(super) fn parse_expression(&mut self) -> Option<Expression<'a>> {
        let expression = match self.read() {
            Token::Identifier { string } => {
                Expression::Identifier(self.parse_identifier(string))
            },
            Token::Integer { string } => {
                Expression::Integer(self.parse_integer(string)?)
            }
            // replace with `token if let Some(operator) = Operator::from(token)`
            // after `if let match guard` is stablized (rfc #2294)
            token if Operator::from(token).is_some() => {
                Expression::Prefix(self.parse_prefix(Operator::from(token)?)?)
            }
            _ => return None
        };

        Some(expression)
    }

    fn parse_identifier(&self, string: &'a str) -> Identifier<'a> {
        Identifier { name: string }
    }

    fn parse_integer(&self, string: &'a str) -> Option<Integer> {
        let value = string.parse::<i64>().ok()?;

        Some(Integer { value })
    }
    
    fn parse_prefix(&mut self, operator: Operator) -> Option<Prefix<'a>> {
        Some(Prefix {
            operator,
            expression: Box::new(self.parse_expression()?),
        })
    }
}