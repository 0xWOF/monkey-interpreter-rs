use std::mem::discriminant;

use crate::{ast::statement::Statement, token::{Token, TokenDiscriminant}};

use super::{Parser, error::ParserError};

impl<'a> Parser<'a> {
    pub(super) fn parse_statement(&mut self) -> Result<Statement<'a>, ParserError> {
        match self.cursor {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement<'a>, ParserError> {
        let name = self.parse_token(TokenDiscriminant::Identifier)?.into_identifier().unwrap();

        self.parse_token(TokenDiscriminant::Assign)?;

        panic!("not implemented")
    }
    
    fn parse_return_statement(&self) -> Result<Statement<'a>, ParserError> {
        self.parse_token(TokenDiscriminant::Return)?;

        panic!("not implemented")
    }

    fn parse_expression_statement(&self) -> Result<Statement<'a>, ParserError> {
        panic!("not implemented")
    }
}

enum A {
    First,
    Second(i32),
}

fn a() {
    let a = A::Second;
    let b = discriminant(&a);
}