use crate::lexer::{Lexer, Token, TokenType};

#[test]
fn test_number_token() {
    let mut lexer = Lexer::new();
    lexer.init("12");

    let next_token = lexer.next_token();

    assert_eq!(
        next_token,
        Token {
            index: 0,
            token_type: TokenType::Number,
            value: "12".to_string(),
        }
    );
}

#[test]
fn test_skip_whitespace() {
    let mut lexer = Lexer::new();
    lexer.init("    12");

    let next_token = lexer.next_token();

    assert_eq!(
        next_token,
        Token {
            index: 4,
            token_type: TokenType::Number,
            value: "12".to_string(),
        }
    );
}

#[test]
fn test_string_tokens() {
    let mut lexer = Lexer::new();
    lexer.init(r#""Hello" 'world'"#);

    let next_token_a = lexer.next_token();
    let next_token_b = lexer.next_token();

    assert_eq!(
        next_token_a,
        Token {
            index: 0,
            token_type: TokenType::String,
            value: "Hello".to_string(),
        }
    );
    assert_eq!(
        next_token_b,
        Token {
            index: 8,
            token_type: TokenType::String,
            value: "world".to_string(),
        }
    );
}
