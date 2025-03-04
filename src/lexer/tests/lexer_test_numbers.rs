use crate::lexer::{Lexer, Token, TokenType};

#[test]
fn test_number_token() {
    let source = "12";
    let mut lexer = Lexer::new(&source);

    let next_token = lexer.next_token();

    assert_eq!(
        next_token,
        Token {
            token_type: TokenType::Number,
            i: source.find('1').unwrap_or(0),
            j: source.find('2').unwrap_or(0) + 1,
        }
    );
}

#[test]
fn test_skip_whitespace() {
    let source = "    12";
    let mut lexer = Lexer::new(source);

    let next_token = lexer.next_token();

    assert_eq!(
        next_token,
        Token {
            token_type: TokenType::Number,
            i: source.find('1').unwrap_or(0),
            j: source.find('2').unwrap_or(0) + 1,
        }
    );
}
