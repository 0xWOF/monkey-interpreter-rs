use crate::{lexer::Lexer, token::Token, ast::program::Program};

pub struct Parser<'a> {
    pub(super) lexer: Lexer<'a>,
    pub(super) cursor: Token<'a>,
    pub(super) peek: Token<'a>,
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

    pub(super) fn read(&mut self) -> Token<'a> {
        self.cursor = self.peek;
        self.peek = self.lexer.next();

        self.cursor
    }
}