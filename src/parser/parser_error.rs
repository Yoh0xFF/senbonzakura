use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Lexical error: {message}")]
    LexicalError { message: String },

    #[error("Parser error: {message}")]
    ParserError { message: String },

    #[error("Type error: {message}")]
    TypeError { message: String },

    #[error("Semantic error: {message}")]
    SemanticError { message: String },
}

pub type ParserResult<T> = Result<T, ParserError>;
