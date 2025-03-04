use crate::lexer::{Lexer, Token, TokenType};

#[test]
fn test_string_tokens() {
    let source = r#"  "Hello" 'world'  "#;
    let token_a = "\"Hello\"";
    let token_b = "'world'";
    let mut lexer = Lexer::new(&source);

    let next_token_a = lexer.next_token();
    let next_token_b = lexer.next_token();

    assert_eq!(
        next_token_a,
        Token {
            token_type: TokenType::String,
            i: source.find(token_a).unwrap_or(0),
            j: source.find(token_a).unwrap_or(0) + token_a.len(),
        }
    );
    assert_eq!(
        next_token_b,
        Token {
            token_type: TokenType::String,
            i: source.find(token_b).unwrap_or(0),
            j: source.find(token_b).unwrap_or(0) + token_a.len(),
        }
    );
}

#[test]
fn test_string_token_with_whitespace() {
    let source = r#"  " Hello "  "#;
    let token = "\" Hello \"";
    let mut lexer = Lexer::new(source);

    let next_token = lexer.next_token();

    assert_eq!(
        next_token,
        Token {
            token_type: TokenType::String,
            i: source.find(token).unwrap_or(0),
            j: source.find(token).unwrap_or(0) + token.len(),
        }
    );
}
