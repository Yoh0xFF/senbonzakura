use std::sync::OnceLock;

use regex::Regex;

use super::token::TokenType;

pub(super) static REGEX_RULES: OnceLock<Vec<(Regex, TokenType)>> = OnceLock::new();

pub(super) fn init_regex_rules() -> Vec<(Regex, TokenType)> {
    let whitespace = Regex::new(r"^\s+").unwrap();
    let number = Regex::new(r"^\d+").unwrap();
    let string_double_quotes = Regex::new(r#"^"[^"]*""#).unwrap();
    let string_single_quotes = Regex::new(r"^'[^']*'").unwrap();

    vec![
        (whitespace, TokenType::Whitespace),
        (number, TokenType::Number),
        (string_double_quotes, TokenType::String),
        (string_single_quotes, TokenType::String),
    ]
}
