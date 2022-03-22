use crate::{ast::statement::{Statement, r#let::Let, expression::Expression}, token::Token};

use super::Parser;

impl<'a> Parser<'a> {
    pub(super) fn parseStatement(&mut self) -> Statement<'a> {
        match self.cursor {
            Token::Let => Statement::Let(self.parseLetStatement()),
            _ => Statement::Expression(self.parseExpressionStatement()),
        }
    }

    pub(super) fn parseLetStatement(&mut self) -> Let<'a> {
        let name = match self.peek {
            Token::Identifier { string } => { self.read(); string },
            _ => unreachable!(),
        };

        match self.peek {
            Token::Assign => self.read(),
            _ => unreachable!(),
        };

        panic!("not implemented")
    }

    pub(super) fn parseExpressionStatement(&self) -> Expression<'a> {
        panic!("not implemented")
    }
}