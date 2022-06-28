use crate::token::TokenDiscriminant;

use super::Parser;

impl<'a> Parser<'a> {
    pub(super) fn report_parse_error(&mut self) {
        self.errors.push("parse")
    }

    pub(super) fn report_unexpected_token(&mut self, expect: &'a str) {
        self.errors.push(
            format!("expect {:?} token, got {:?} token", expect, self.peek).as_str()
        )
    }

    pub(super) fn error_unexpected_token(&mut self, expect: TokenDiscriminant) -> ParserError<'a> {
        ParserError {
            message: format!("expect {:?} token, but got {:?} token", expect, self.peek).as_str(),
        }
    }
}

pub struct ParserError<'a> {
    message: &'a str,
}