use crate::token::{TokenDiscriminant, Token};

use super::{Parser, error::ParserError};

impl<'a> Parser<'a> {
    pub(super) fn parse_token(&mut self, expect: TokenDiscriminant) -> Result<Token, ParserError> {
        match self.peek {
            token if TokenDiscriminant::from(token) == expect => Ok(self.read()),
            _ => return Err(self.error_unexpected_token(expect)),
        }
    }
}