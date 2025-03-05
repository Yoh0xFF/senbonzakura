use std::sync::OnceLock;

use regex::Regex;

use super::token::TokenType;

pub(super) static REGEX_RULES: OnceLock<Vec<(Regex, TokenType)>> = OnceLock::new();

pub(super) fn init_regex_rules() -> Vec<(Regex, TokenType)> {
    // Whitespace
    let whitespace = Regex::new(r"^\s+").unwrap();

    // Comments
    let single_line_comments = Regex::new(r"^//.*").unwrap();
    let multi_line_comments = Regex::new(r"^/\*[\s\S]*?\*/").unwrap();

    // Symbols, delimiters
    let statement_end = Regex::new(r"^;").unwrap();

    // Numbers
    let number = Regex::new(r"^\d+").unwrap();

    // Strings
    let string_double_quotes = Regex::new(r#"^"[^"]*""#).unwrap();
    let string_single_quotes = Regex::new(r"^'[^']*'").unwrap();

    vec![
        (whitespace, TokenType::Whitespace),
        (single_line_comments, TokenType::SingleLineComment),
        (multi_line_comments, TokenType::MultiLineComment),
        (statement_end, TokenType::StatementEnd),
        (number, TokenType::Number),
        (string_double_quotes, TokenType::String),
        (string_single_quotes, TokenType::String),
    ]
}
