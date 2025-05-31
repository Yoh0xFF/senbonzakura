use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("Unexpected character '{char}' at line {line}, column {column}")]
    UnexpectedCharacter {
        char: char,
        line: usize,
        column: usize,
    },

    #[error("Unterminated string literal at line {line}, column {column}")]
    UnterminatedString { line: usize, column: usize },

    #[error("Unterminated comment at line {line}, column {column}")]
    UnterminatedComment { line: usize, column: usize },

    #[error("Invalid number format at line {line}, column {column}")]
    InvalidNumber { line: usize, column: usize },
}

pub type LexerResult<T> = Result<T, LexerError>;
