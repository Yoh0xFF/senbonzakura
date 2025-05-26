use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum TokenType {
    // Literals
    Number,
    String,
    BooleanTrue,
    BooleanFalse,
    Nil,

    // Identifiers and Keywords
    Identifier,

    // Control flow keywords
    IfKeyword,
    ElseKeyword,
    WhileKeyword,
    DoKeyword,
    ForKeyword,
    ReturnKeyword,

    // Declaration keywords
    LetKeyword,
    DefKeyword,
    ClassKeyword,
    ExtendsKeyword,

    // Special keywords
    ThisKeyword,
    SuperKeyword,
    NewKeyword,
    TypeKeyword,

    // Type keywords
    NumberTypeKeyword,
    StringTypeKeyword,
    BooleanTypeKeyword,
    VoidTypeKeyword,

    // Operators - Arithmetic
    AdditivePlusOperator,
    AdditiveMinusOperator,
    FactorMultiplicationOperator,
    FactorDivisionOperator,
    ModuloOperator,   // % - new TODO
    ExponentOperator, // ** - new TODO

    // Operators - Assignment
    SimpleAssignmentOperator,
    ComplexPlusAssignmentOperator,
    ComplexMinusAssignmentOperator,
    ComplexMultiplyAssignmentOperator,
    ComplexDivideAssignmentOperator,
    ComplexModuloAssignmentOperator,   // %= - new TODO
    ComplexExponentAssignmentOperator, // **= - new TODO

    // Operators - Comparison
    EqualOperator,
    NotEqualOperator,
    RelationalGreaterThanOperator,
    RelationalGreaterThanOrEqualToOperator,
    RelationalLessThanOperator,
    RelationalLessThanOrEqualToOperator,

    // Operators - Logical
    LogicalAndOperator,
    LogicalOrOperator,
    LogicalNotOperator,

    // Operators - Bitwise (new)
    BitwiseAndOperator, // & TODO
    BitwiseOrOperator,  // | TODO
    BitwiseXorOperator, // ^ TODO
    BitwiseNotOperator, // ~ TODO
    LeftShiftOperator,  // << TODO
    RightShiftOperator, // >> TODO

    // Operators - Unary
    IncrementOperator, // ++ - new TODO
    DecrementOperator, // -- - new TODO

    // Punctuation
    StatementEnd,       // ;
    OpeningBrace,       // {
    ClosingBrace,       // }
    OpeningParenthesis, // (
    ClosingParenthesis, // )
    OpeningBracket,     // [
    ClosingBracket,     // ]
    Comma,              // ,
    Dot,                // .
    Colon,              // :
    QuestionMark,       // ? - new (for ternary operator) TODO
    Arrow,              // => - new (for arrow functions) TODO
    DoubleColon,        // :: - new (for namespacing) TODO

    // Special
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

#[allow(dead_code)]
impl Token {
    pub fn text<'a>(&self, source: &'a str) -> &'a str {
        &source[self.start.offset..self.end.offset]
    }

    pub fn len(&self) -> usize {
        self.end.offset - self.start.offset
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // Helper methods for token classification
    pub fn is_literal(&self) -> bool {
        matches!(
            self.token_type,
            TokenType::Number
                | TokenType::String
                | TokenType::BooleanTrue
                | TokenType::BooleanFalse
                | TokenType::Nil
        )
    }

    pub fn is_keyword(&self) -> bool {
        matches!(
            self.token_type,
            TokenType::IfKeyword
                | TokenType::ElseKeyword
                | TokenType::WhileKeyword
                | TokenType::DoKeyword
                | TokenType::ForKeyword
                | TokenType::ReturnKeyword
                | TokenType::LetKeyword
                | TokenType::DefKeyword
                | TokenType::ClassKeyword
                | TokenType::ExtendsKeyword
                | TokenType::ThisKeyword
                | TokenType::SuperKeyword
                | TokenType::NewKeyword
                | TokenType::TypeKeyword
        )
    }

    pub fn is_type_keyword(&self) -> bool {
        matches!(
            self.token_type,
            TokenType::NumberTypeKeyword
                | TokenType::StringTypeKeyword
                | TokenType::BooleanTypeKeyword
                | TokenType::VoidTypeKeyword
        )
    }

    pub fn is_operator(&self) -> bool {
        matches!(
            self.token_type,
            TokenType::AdditivePlusOperator
                | TokenType::AdditiveMinusOperator
                | TokenType::FactorMultiplicationOperator
                | TokenType::FactorDivisionOperator
                | TokenType::ModuloOperator
                | TokenType::ExponentOperator
                | TokenType::EqualOperator
                | TokenType::NotEqualOperator
                | TokenType::RelationalGreaterThanOperator
                | TokenType::RelationalGreaterThanOrEqualToOperator
                | TokenType::RelationalLessThanOperator
                | TokenType::RelationalLessThanOrEqualToOperator
                | TokenType::LogicalAndOperator
                | TokenType::LogicalOrOperator
                | TokenType::LogicalNotOperator
        )
    }

    pub fn is_assignment_operator(&self) -> bool {
        matches!(
            self.token_type,
            TokenType::SimpleAssignmentOperator
                | TokenType::ComplexPlusAssignmentOperator
                | TokenType::ComplexMinusAssignmentOperator
                | TokenType::ComplexMultiplyAssignmentOperator
                | TokenType::ComplexDivideAssignmentOperator
                | TokenType::ComplexModuloAssignmentOperator
                | TokenType::ComplexExponentAssignmentOperator
        )
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token({}, {}:{}-{}:{})",
            self.token_type, self.start.line, self.start.column, self.end.line, self.end.column
        )
    }
}
