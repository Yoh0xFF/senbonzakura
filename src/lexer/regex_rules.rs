use std::sync::OnceLock;

use regex::Regex;

use super::token::TokenType;

pub(super) static REGEX_RULES: OnceLock<Vec<(Regex, TokenType)>> = OnceLock::new();

pub(super) fn init_regex_rules() -> Vec<(Regex, TokenType)> {
    // Whitespace
    let whitespace = Regex::new(r"^\s+").expect("Failed to compile regex for whitespace");

    // Comments
    let single_line_comments =
        Regex::new(r"^//.*").expect("Failed to compile regex for single line comments");
    let multi_line_comments =
        Regex::new(r"^/\*[\s\S]*?\*/").expect("Failed to compile regex for multi line comments");

    // Symbols, delimiters
    let statement_end =
        Regex::new(r"^;").expect("Failed to compile regex for statement end (;) symbol");
    let opening_brace =
        Regex::new(r"^\{").expect("Failed to compile regex for opening brace ({) symbol");
    let closing_brace =
        Regex::new(r"^\}").expect("Failed to compile regex for closing brace (}) symbol");
    let opening_parenthesis =
        Regex::new(r"^\(").expect("Failed to compile regex for opening parenthesis (() symbol");
    let closing_parenthesis =
        Regex::new(r"^\)").expect("Failed to compile regex for closing parenthesis ()) symbol");
    let comma = Regex::new(r"^,)").expect("Failed to compile regex for comma (,) symbol");

    // Keywords
    let let_keyword =
        Regex::new(r"^\blet\b").expect("Failed to compile regex for the 'let' keyword");

    // Assignment operators
    let simple_assignment_operator =
        Regex::new(r"^=").expect("Failed to compile regex for simgle assignment operator");
    let complex_assignment_operator =
        Regex::new(r"^[*/+-]=").expect("Failed to compile regex for complex assignment operator");

    // Math operators
    let additive_operator =
        Regex::new(r"^[+\-]").expect("Failed to compile regex for additive operators (+, -)");
    let factor_operator =
        Regex::new(r"^[*\/]").expect("Failed to compile regex for factor operators (*, /");

    // Numbers
    let number = Regex::new(r"^\d+").expect("Failed to compile regex for number literal");

    // Strings
    let string_double_quotes =
        Regex::new(r#"^"[^"]*""#).expect("Failed to compile regex for double quote string literal");
    let string_single_quotes =
        Regex::new(r"^'[^']*'").expect("Failed to compile regex for single quote string literal");

    // Identifiers
    let identifier = Regex::new(r"^\w+").expect("Failed to compile regex for identifiers");

    vec![
        (whitespace, TokenType::Whitespace),
        (single_line_comments, TokenType::SingleLineComment),
        (multi_line_comments, TokenType::MultiLineComment),
        (statement_end, TokenType::StatementEnd),
        (opening_brace, TokenType::OpeningBrace),
        (closing_brace, TokenType::ClosingBrace),
        (opening_parenthesis, TokenType::OpeningParenthesis),
        (closing_parenthesis, TokenType::ClosingParenthesis),
        (comma, TokenType::Comma),
        (let_keyword, TokenType::LetKeyword),
        (
            simple_assignment_operator,
            TokenType::SimpleAssignmentOperator,
        ),
        (
            complex_assignment_operator,
            TokenType::ComplexAssignmentOperator,
        ),
        (additive_operator, TokenType::AdditiveOperator),
        (factor_operator, TokenType::FactorOperator),
        (number, TokenType::Number),
        (string_double_quotes, TokenType::String),
        (string_single_quotes, TokenType::String),
        // Important! Order matters this rule must be after the number literal rule
        (identifier, TokenType::Identifier),
    ]
}
