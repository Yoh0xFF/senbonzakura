use core::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
    Comma,
    // Keywords,
    LetKeyword,
    IfKeyword,
    ElseKeyword,
    // Identifier
    Identifier,
    // Equality operators
    EqualityOperator,
    // Assignment operators
    SimpleAssignmentOperator,
    ComplexAssignmentOperator,
    // Math operators,
    AdditiveOperator,
    FactorOperator,
    RelationalOperator,
    // Literals
    Boolean,
    Nil,
    Number,
    String,
    // End
    End,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub i: usize,
    pub j: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Token ({}, {}, {})",
            self.token_type.to_string(),
            self.i,
            self.j,
        )
    }
}
