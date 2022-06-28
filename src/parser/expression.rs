use crate::{ast::expression::{identifier::Identifier, Expression, prefix::{Prefix, Operator}, integer::Integer}, token::Token};

use super::Parser;

impl<'a> Parser<'a> {
    pub(super) fn parse_expression(&mut self) -> Option<Expression<'a>> {
        let expression = match self.read() {
            Token::Identifier(name) => {
                Expression::Identifier(self.parse_identifier(name))
            },
            Token::Integer(value) => {
                Expression::Integer(self.parse_integer(value))
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

    fn parse_identifier(&self, name: &'a str) -> Identifier<'a> {
        Identifier { name }
    }

    fn parse_integer(&self, value: u64) -> Integer {
        Integer { value }
    }
    
    fn parse_prefix(&mut self, operator: Operator) -> Option<Prefix<'a>> {
        Some(Prefix {
            operator,
            expression: Box::new(self.parse_expression()?),
        }).or_else(|| {
            self.report_parse_error();
            None
        })
    }
}