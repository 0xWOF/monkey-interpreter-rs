#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Illegal,
    EOF,

    Identifier { string: &'a str },
    Integer,

    Assign,
    Plus,

    Comma,
    Semicolon,

    LeftParenthesis,
    RightParenthesis,
    LeftBracket,
    RightBracket,

    Function,
    Let,
}