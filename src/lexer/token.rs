use core::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    Whitespace,
    Number,
    String,
    End,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub index: usize,
    pub token_type: TokenType,
    pub value: String,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Token ({}, {}, {})",
            self.index,
            self.token_type.to_string(),
            self.value
        )
    }
}
