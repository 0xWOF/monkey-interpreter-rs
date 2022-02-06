use monkey_interpreter_rs::token::*;
use monkey_interpreter_rs::lexer::*;

#[test]
fn test_next_token() {
    let input = "=+(){},;";
    let expects = [
        Token::Assign,
        Token::Plus,
        Token::LeftParenthesis,
        Token::RightParenthesis,
        Token::LeftBracket,
        Token::RightBracket,
        Token::Comma,
        Token::Semicolon,
        Token::EOF,
    ];

    let mut lexer = Lexer::new(input);

    for expect in expects.iter() {
        assert_eq!(lexer.next(), *expect)
    }
}