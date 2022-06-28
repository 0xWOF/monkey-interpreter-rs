use strum_macros::EnumDiscriminants;
use enum_as_inner::EnumAsInner;

#[derive(PartialEq, Debug, Clone, Copy)]
#[derive(EnumDiscriminants)]
#[derive(EnumAsInner)]
#[strum_discriminants(vis(pub))]
#[strum_discriminants(name(TokenDiscriminant))]
pub enum Token<'a> {
    Illegal,
    EOF,

    Identifier(&'a str),
    Integer(u64),

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