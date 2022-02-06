use std::str::Chars;

use crate::token::Token;

pub struct Lexer<'a> {
    input: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer {
        Lexer { input: input.chars() }
    }

    pub fn next(&mut self) -> Token {
        match self.read() {
            '=' => Token::Assign,
            ';' => Token::Semicolon,
            '(' => Token::LeftParenthesis,
            ')' => Token::RightParenthesis,
            ',' => Token::Comma,
            '+' => Token::Plus,
            '{' => Token::LeftBracket,
            '}' => Token::RightBracket,
            '\0' => Token::EOF,
            _ => {
                Token::Identifier { string: "123" }
            },
        }
    }

    fn read(&mut self) -> char {
        self.input.next().unwrap_or('\0')
    }

    fn read_identifier(&mut self) -> &str {
        self.input

        "123"
    }
}