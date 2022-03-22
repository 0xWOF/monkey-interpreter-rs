use crate::token::Token;

use crate::utility::check::{is_letter, is_digit, is_space};

pub struct Lexer<'a> {
    input: &'a str,
    cursor: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        Lexer { input, cursor: 0 }
    }

    pub fn next(&mut self) -> Token<'a> {
        match self.read() {
            '=' => match self.peek() {
                '=' => { self.read(); Token::Equal },
                _ => Token::Assign,
            },
            '+' => Token::Plus,
            '-' => Token::Minus,
            '!' => match self.peek() {
                '=' => { self.read(); Token::NotEqual },
                _ => Token::Exclamation,
            },
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            '<' => Token::LessThan,
            '>' => Token::GreaterThan,
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            '(' => Token::LeftParenthesis,
            ')' => Token::RightParenthesis,
            '{' => Token::LeftBracket,
            '}' => Token::RightBracket,
            '\0' => Token::EOF,
            x if is_space(x) => self.next(),
            x if is_letter(x) => self.next_identifier(),
            x if is_digit(x) => self.next_digit(),
            _ => Token::Illegal
        }
    }

    fn next_identifier(&mut self) -> Token<'a> {
        let identifier = self.read_identifier();

        match identifier {
            "fn" => Token::Function,
            "let" => Token::Let,
            "true" => Token::True,
            "false" => Token::False,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            "" => Token::Illegal,
            _ => Token::Identifier { string: identifier },
        }
    }

    fn next_digit(&mut self) -> Token<'a> {
        Token::Integer { string: self.read_digit() }
    }

    fn peek(&self) -> char {
        self.input
        .chars()
        .nth(self.cursor)
        .unwrap_or('\0')
    }

    fn read(&mut self) -> char {
        let letter = self.peek();
        self.cursor += 1;

        return letter;
    }

    fn back(&mut self) {
        self.cursor -= 1;
    }

    fn read_identifier(&mut self) -> &'a str {
        let start = self.cursor - 1;
        while is_letter(self.read()) {}
        let end = self.cursor - 1;

        self.back();

        &self.input[start..end]
    }

    fn read_digit(&mut self) -> &'a str {
        let start = self.cursor - 1;
        while is_digit(self.read()) {}
        let end = self.cursor - 1;

        self.back();

        &self.input[start..end]
    }
}