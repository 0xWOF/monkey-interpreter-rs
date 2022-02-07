use monkey_interpreter_rs::token::*;
use monkey_interpreter_rs::lexer::*;

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
        Token::Identifier { string: "five" },
        Token::Assign,
        Token::Integer { string: "5" },
        Token::Semicolon,
        Token::Let,
        Token::Identifier { string: "ten" },
        Token::Assign,
        Token::Integer { string: "10" },
        Token::Semicolon,
        Token::Let,
        Token::Identifier { string: "add" },
        Token::Assign,
        Token::Function,
        Token::LeftParenthesis,
        Token::Identifier { string: "x" },
        Token::Comma,
        Token::Identifier { string: "y" },
        Token::RightParenthesis,
        Token::LeftBracket,
        Token::Identifier { string: "x" },
        Token::Plus,
        Token::Identifier { string: "y" },
        Token::Semicolon,
        Token::RightBracket,
        Token::Semicolon,
        Token::Let,
        Token::Identifier { string: "result" },
        Token::Assign,
        Token::Identifier { string: "add" },
        Token::LeftParenthesis,
        Token::Identifier { string: "five" },
        Token::Comma,
        Token::Identifier { string: "ten" },
        Token::RightParenthesis,
        Token::Semicolon,
        Token::Exclamation,
        Token::Minus,
        Token::Slash,
        Token::Asterisk,
        Token::Integer { string: "5" },
        Token::Semicolon,
        Token::Integer { string: "5" },
        Token::LessThan,
        Token::Integer { string: "10" },
        Token::GreaterThan,
        Token::Integer { string: "5" },
        Token::Semicolon,
        Token::If,
        Token::LeftParenthesis,
        Token::Integer { string: "5" },
        Token::LessThan,
        Token::Integer { string: "10" },
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
        Token::Integer { string: "10" },
        Token::Equal,
        Token::Assign,
        Token::Integer { string: "10" },
        Token::Semicolon,
        Token::Integer { string: "10" },
        Token::NotEqual,
        Token::Assign,
        Token::Integer { string: "9" },
        Token::Semicolon,
    ];

    let mut lexer = Lexer::new(input);

    for expect in expects.iter() {
        assert_eq!(lexer.next(), *expect);
    }
}