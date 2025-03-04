use crate::lexer::{Lexer, Token, TokenType};

#[test]
fn test_single_line_comment() {
    let source = r"
        // This is single line comment
        17
    ";

    let mut lexer = Lexer::new(&source);

    let next_token = lexer.next_token();

    assert_eq!(
        next_token,
        Token {
            token_type: TokenType::Number,
            i: source.find('1').unwrap_or(0),
            j: source.find('7').unwrap_or(0) + 1,
        }
    );
}

#[test]
fn test_multi_line_comment() {
    let source = r"
        /*
        This is multi line comment,
        and we should skip it
        */
        1719
    ";

    let mut lexer = Lexer::new(source);

    let next_token = lexer.next_token();

    assert_eq!(
        next_token,
        Token {
            token_type: TokenType::Number,
            i: source.find('1').unwrap_or(0),
            j: source.find('9').unwrap_or(0) + 1,
        }
    );
}

#[test]
fn test_formatted_multi_line_comment() {
    let source = r"
        /**
        * This is multi line comment,
        * and we should skip it
        */
        1719
    ";

    let mut lexer = Lexer::new(source);

    let next_token = lexer.next_token();

    assert_eq!(
        next_token,
        Token {
            token_type: TokenType::Number,
            i: source.find('1').unwrap_or(0),
            j: source.find('9').unwrap_or(0) + 1,
        }
    );
}
