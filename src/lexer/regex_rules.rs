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
        Regex::new(r"^}").expect("Failed to compile regex for closing brace (}) symbol");
    let opening_parenthesis =
        Regex::new(r"^\(").expect("Failed to compile regex for opening parenthesis (() symbol");
    let closing_parenthesis =
        Regex::new(r"^\)").expect("Failed to compile regex for closing parenthesis ()) symbol");
    let comma = Regex::new(r"^,").expect("Failed to compile regex for comma (,) symbol");

    // Keywords
    let true_keyword =
        Regex::new(r"^\btrue\b").expect("Failed to compile regex for the 'true' keyword");
    let false_keyword =
        Regex::new(r"^\bfalse\b").expect("Failed to compile regex for the 'false' keyword");
    let nil_keyword =
        Regex::new(r"^\bnil\b").expect("Failed to compile regex for the 'nil' keyword");
    let let_keyword =
        Regex::new(r"^\blet\b").expect("Failed to compile regex for the 'let' keyword");
    let if_keyword = Regex::new(r"^\bif\b").expect("Failed to compile regex for the 'if' keyword");
    let else_keyword =
        Regex::new(r"^\belse\b").expect("Failed to compile regex for the 'else' keyword");
    let while_keyword =
        Regex::new(r"^\bwhile\b").expect("Failed to compile regex for the 'while' keyword");
    let do_keyword = Regex::new(r"^\bdo\b").expect("Failed to compile regex for the 'do' keyword");
    let for_keyword =
        Regex::new(r"^\bfor\b").expect("Failed to compile regex for the 'for' keyword");

    // Equality Operator
    let equality_operator =
        Regex::new(r"^[=!]=").expect("Failed to compile regex for equality operator");

    // Assignment operators
    let simple_assignment_operator =
        Regex::new(r"^=").expect("Failed to compile regex for single assignment operator");
    let complex_assignment_operator =
        Regex::new(r"^[*/+-]=").expect("Failed to compile regex for complex assignment operator");

    // Math operators
    let additive_operator =
        Regex::new(r"^[+\-]").expect("Failed to compile regex for additive operators (+, -)");
    let factor_operator =
        Regex::new(r"^[*/]").expect("Failed to compile regex for factor operators (*, /");

    // Relational operators
    let relational_operator = Regex::new(r"^[><]=?")
        .expect("Failed to compile regex for relational operators (>, >=, <, <=)");

    // Logical operators
    let logical_and_operator =
        Regex::new(r"^&&").expect("Failed to compile regex for logical and operator");
    let logical_or_operator =
        Regex::new(r"^\|\|").expect("Failed to compile regex for logical or operator");
    let logical_not_operator =
        Regex::new(r"^!").expect("Failed to compile regex for logical not operator");

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
        (true_keyword, TokenType::Boolean),
        (false_keyword, TokenType::Boolean),
        (nil_keyword, TokenType::Nil),
        (let_keyword, TokenType::LetKeyword),
        (if_keyword, TokenType::IfKeyword),
        (else_keyword, TokenType::ElseKeyword),
        (while_keyword, TokenType::WhileKeyword),
        (do_keyword, TokenType::DoKeyword),
        (for_keyword, TokenType::ForKeyword),
        // Important! Order matters this rule must be before the assignment operators
        (equality_operator, TokenType::EqualityOperator),
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
        (relational_operator, TokenType::RelationalOperator),
        (logical_and_operator, TokenType::LogicalAndOperator),
        (logical_or_operator, TokenType::LogicalOrOperator),
        (logical_not_operator, TokenType::LogicalNotOperator),
        (number, TokenType::Number),
        (string_double_quotes, TokenType::String),
        (string_single_quotes, TokenType::String),
        // Important! Order matters this rule must be after the number literal rule
        (identifier, TokenType::Identifier),
    ]
}
