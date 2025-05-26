use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum TokenType {
    // Whitespace and comments
    Whitespace,
    SingleLineComment,
    MultiLineComment,

    // Symbols
    StatementEnd,
    OpeningBrace,
    ClosingBrace,
    OpeningParenthesis,
    ClosingParenthesis,
    OpeningBracket,
    ClosingBracket,
    Comma,
    Dot,
    Colon,

    // Keywords,
    LetKeyword,
    IfKeyword,
    ElseKeyword,
    WhileKeyword,
    DoKeyword,
    ForKeyword,
    DefKeyword,
    ReturnKeyword,
    ClassKeyword,
    ExtendsKeyword,
    ThisKeyword,
    SuperKeyword,
    NewKeyword,
    TypeKeyword,
    NumberTypeKeyword,
    StringTypeKeyword,
    BooleanTypeKeyword,
    VoidTypeKeyword,

    // Identifier
    Identifier,

    // Equality operators
    EqualOperator,
    NotEqualOperator,

    // Assignment operators
    SimpleAssignmentOperator,
    ComplexPlusAssignmentOperator,
    ComplexMinusAssignmentOperator,
    ComplexMultiplyAssignmentOperator,
    ComplexDivideAssignmentOperator,

    // Math operators
    AdditivePlusOperator,
    AdditiveMinusOperator,
    FactorMultiplicationOperator,
    FactorDivisionOperator,

    // Relational operators
    RelationalGreaterThanOperator,
    RelationalGreaterThanOrEqualToOperator,
    RelationalLessThanOperator,
    RelationalLessThanOrEqualToOperator,

    // Logical operators
    LogicalAndOperator,
    LogicalOrOperator,
    LogicalNotOperator,

    // Literals
    BooleanTrue,
    BooleanFalse,
    Nil,
    Number,
    String,

    // End
    End,
    Unknown,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

///
/// Token position in the source
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenPosition {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

impl TokenPosition {
    pub fn new() -> Self {
        TokenPosition {
            line: 1,
            column: 1,
            offset: 0,
        }
    }

    pub fn advance(&mut self, ch: char) {
        self.offset += ch.len_utf8();
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
    }
}

///
/// Token structure
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Token {
    pub token_type: TokenType,
    pub start: TokenPosition,
    pub end: TokenPosition,
}

impl Token {
    pub fn text<'a>(&self, source: &'a str) -> &'a str {
        &source[self.start.offset..self.end.offset]
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token ({}, {:?}, {:?})",
            self.token_type.to_string(),
            self.start,
            self.end,
        )
    }
}
