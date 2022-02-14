use crate::{lexer::Lexer, token::Token, ast::{program::Program, statement::{Statement, r#let::Let}, expression::{identifier::Identifier, Expression}}};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    cursor: Token<'a>,
    peek: Token<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Parser<'a> {
        let cursor = lexer.next();
        let peek = lexer.next();

        Parser {
            lexer,
            cursor,
            peek,
        }
    }

    pub fn parse(&mut self) -> Program<'a> {
        let mut program = Program::new();

        while self.cursor != Token::EOF {
            program.statements.push(
                self.parseStatement()
            );
            self.read();
        }

        program
    }

    fn parseStatement(&self) -> Statement<'a> {
        match self.cursor {
            Token::Let => Statement::Let(self.parseLetStatement()),
            _ => unreachable!(),
        }
    }

    fn parseLetStatement(&self) -> Let<'a> {
        panic!("not implemented")
    }

    fn parseIdentifier(&self) -> Identifier<'a> {
        panic!("not implemented")
    }

    fn parseExpression(&self) -> Expression<'a> {
        panic!("not implemented")
    }

    fn read(&mut self) {
        self.cursor = self.peek;
        self.peek = self.lexer.next();
    }
}