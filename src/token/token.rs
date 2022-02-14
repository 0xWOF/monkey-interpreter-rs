#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Token<'a> {
    Illegal,
    EOF,

    Identifier { string: &'a str },
    Integer { string: &'a str },

    Assign,
    Plus,
    Minus,
    Exclamation,
    Asterisk,
    Slash,
    LessThan,
    GreaterThan,
    Comma,
    Semicolon,
    LeftParenthesis,
    RightParenthesis,
    LeftBracket,
    RightBracket,
    Equal,
    NotEqual,

    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}