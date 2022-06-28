use crate::token::*;
use crate::lexer::*;

#[test]
fn test_next_token() {
    let input = "
        let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 != 9;
    ";
    let expects = [
        Token::Let,
        Token::Identifier("five"),
        Token::Assign,
        Token::Integer(5),
        Token::Semicolon,
        Token::Let,
        Token::Identifier("ten"),
        Token::Assign,
        Token::Integer(10),
        Token::Semicolon,
        Token::Let,
        Token::Identifier("add"),
        Token::Assign,
        Token::Function,
        Token::LeftParenthesis,
        Token::Identifier("x"),
        Token::Comma,
        Token::Identifier("y"),
        Token::RightParenthesis,
        Token::LeftBracket,
        Token::Identifier("x"),
        Token::Plus,
        Token::Identifier("y"),
        Token::Semicolon,
        Token::RightBracket,
        Token::Semicolon,
        Token::Let,
        Token::Identifier("result"),
        Token::Assign,
        Token::Identifier("add"),
        Token::LeftParenthesis,
        Token::Identifier("five"),
        Token::Comma,
        Token::Identifier("ten"),
        Token::RightParenthesis,
        Token::Semicolon,
        Token::Exclamation,
        Token::Minus,
        Token::Slash,
        Token::Asterisk,
        Token::Integer(5),
        Token::Semicolon,
        Token::Integer(5),
        Token::LessThan,
        Token::Integer(10),
        Token::GreaterThan,
        Token::Integer(5),
        Token::Semicolon,
        Token::If,
        Token::LeftParenthesis,
        Token::Integer(5),
        Token::LessThan,
        Token::Integer(10),
        Token::RightParenthesis,
        Token::LeftBracket,
        Token::Return,
        Token::True,
        Token::Semicolon,
        Token::RightBracket,
        Token::Else,
        Token::LeftBracket,
        Token::Return,
        Token::False,
        Token::Semicolon,
        Token::RightBracket,
        Token::Integer(10),
        Token::Equal,
        Token::Integer(10),
        Token::Semicolon,
        Token::Integer(10),
        Token::NotEqual,
        Token::Integer(9),
        Token::Semicolon,
    ];

    let mut lexer = Lexer::new(input);

    for expect in expects.iter() {
        assert_eq!(lexer.next(), *expect);
    }
}