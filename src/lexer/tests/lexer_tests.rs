use crate::lexer::{Lexer, Token, TokenType};

#[test]
fn test_number_token() {
    let mut lexer = Lexer::new("12");

    let next_token = lexer.next_token();

    assert_eq!(
        next_token,
        Token {
            token_type: TokenType::Number,
            i: 0,
            j: 2
        }
    );
}

#[test]
fn test_skip_whitespace() {
    let mut lexer = Lexer::new("    12");

    let next_token = lexer.next_token();

    assert_eq!(
        next_token,
        Token {
            token_type: TokenType::Number,
            i: 4,
            j: 6,
        }
    );
}

#[test]
fn test_string_tokens() {
    let mut lexer = Lexer::new(r#""Hello" 'world'"#);

    let next_token_a = lexer.next_token();
    let next_token_b = lexer.next_token();

    assert_eq!(
        next_token_a,
        Token {
            token_type: TokenType::String,
            i: 0,
            j: 7,
        }
    );
    assert_eq!(
        next_token_b,
        Token {
            token_type: TokenType::String,
            i: 8,
            j: 15,
        }
    );
}
