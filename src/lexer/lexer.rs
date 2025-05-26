use super::{
    token::{Token, TokenType},
    TokenPosition,
};
use thiserror::Error;

///
/// Lexer errors
///
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
    #[allow(dead_code)]
    InvalidNumber { line: usize, column: usize },
}

///
/// Lexer class
/// Lazily pulls a token from a stream
///
#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    source: &'a str,
    // index: usize,
    position: TokenPosition,
    chars: std::str::CharIndices<'a>,
    current_char: Option<(usize, char)>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut chars = source.char_indices();
        let current_char = chars.next();

        Lexer {
            source,
            position: TokenPosition::new(),
            chars,
            current_char,
        }
    }

    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        self.skip_whitespace_and_comments()?;

        let Some(ch) = self.peek_char() else {
            return Ok(Token {
                token_type: TokenType::End,
                start: self.position,
                end: self.position,
            });
        };

        let start_pos = self.position;

        match ch {
            // Match strings
            '"' | '\'' => self.read_string(ch),

            // Match numbers
            '0'..='9' => self.read_number(),

            // Match keywords and identifiers
            'a'..='z' | 'A'..='Z' | '_' => Ok(self.read_identifier()),

            // Match symbols
            ';' | ':' | '.' | ',' | '{' | '}' | '(' | ')' | '[' | ']' => {
                let token_type = match ch {
                    ';' => TokenType::StatementEnd,
                    ':' => TokenType::Colon,
                    '.' => TokenType::Dot,
                    ',' => TokenType::Comma,
                    '{' => TokenType::OpeningBrace,
                    '}' => TokenType::ClosingBrace,
                    '(' => TokenType::OpeningParenthesis,
                    ')' => TokenType::ClosingParenthesis,
                    '[' => TokenType::OpeningBracket,
                    ']' => TokenType::ClosingBracket,
                    _ => TokenType::Unknown,
                };

                if token_type == TokenType::Unknown {
                    return Err(LexerError::UnexpectedCharacter {
                        char: ch,
                        line: start_pos.line,
                        column: start_pos.column,
                    });
                }

                self.advance();
                Ok(Token {
                    token_type,
                    start: start_pos,
                    end: self.position,
                })
            }

            // Match multi-character operators
            '=' => {
                self.advance();

                let token_type = if self.peek_char() == Some('=') {
                    self.advance();
                    TokenType::EqualOperator
                } else {
                    TokenType::SimpleAssignmentOperator
                };

                Ok(Token {
                    token_type,
                    start: start_pos,
                    end: self.position,
                })
            }
            '+' | '-' | '*' | '/' => {
                self.advance();

                let token_type = if self.peek_char() == Some('=') {
                    self.advance();
                    match ch {
                        '+' => TokenType::ComplexPlusAssignmentOperator,
                        '-' => TokenType::ComplexMinusAssignmentOperator,
                        '*' => TokenType::ComplexMultiplyAssignmentOperator,
                        '/' => TokenType::ComplexDivideAssignmentOperator,
                        _ => TokenType::Unknown,
                    }
                } else {
                    match ch {
                        '+' => TokenType::AdditivePlusOperator,
                        '-' => TokenType::AdditiveMinusOperator,
                        '*' => TokenType::FactorMultiplicationOperator,
                        '/' => TokenType::FactorDivisionOperator,
                        _ => TokenType::Unknown,
                    }
                };

                if token_type == TokenType::Unknown {
                    return Err(LexerError::UnexpectedCharacter {
                        char: ch,
                        line: start_pos.line,
                        column: start_pos.column,
                    });
                }

                Ok(Token {
                    token_type,
                    start: start_pos,
                    end: self.position,
                })
            }
            '>' => {
                self.advance();

                let token_type = if self.peek_char() == Some('=') {
                    self.advance();
                    TokenType::RelationalGreaterThanOrEqualToOperator
                } else {
                    TokenType::RelationalGreaterThanOperator
                };

                Ok(Token {
                    token_type,
                    start: start_pos,
                    end: self.position,
                })
            }
            '<' => {
                self.advance();

                let token_type = if self.peek_char() == Some('=') {
                    self.advance();
                    TokenType::RelationalLessThanOrEqualToOperator
                } else {
                    TokenType::RelationalLessThanOperator
                };

                Ok(Token {
                    token_type,
                    start: start_pos,
                    end: self.position,
                })
            }
            '&' => {
                self.advance();

                if self.peek_char() == Some('&') {
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::LogicalAndOperator,
                        start: start_pos,
                        end: self.position,
                    })
                } else {
                    Err(LexerError::UnexpectedCharacter {
                        char: ch,
                        line: start_pos.line,
                        column: start_pos.column,
                    })
                }
            }
            '|' => {
                self.advance();

                if self.peek_char() == Some('|') {
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::LogicalOrOperator,
                        start: start_pos,
                        end: self.position,
                    })
                } else {
                    Err(LexerError::UnexpectedCharacter {
                        char: ch,
                        line: start_pos.line,
                        column: start_pos.column,
                    })
                }
            }
            '!' => {
                self.advance();

                let token_type = if self.peek_char() == Some('=') {
                    self.advance();
                    TokenType::NotEqualOperator
                } else {
                    TokenType::LogicalNotOperator
                };

                Ok(Token {
                    token_type,
                    start: start_pos,
                    end: self.position,
                })
            }

            // Add more operators...
            _ => Err(LexerError::UnexpectedCharacter {
                char: ch,
                line: start_pos.line,
                column: start_pos.column,
            }),
        }
    }

    #[allow(dead_code)]
    pub fn peek_token(&mut self) -> Result<Token, LexerError> {
        // Save current state
        let saved_position = self.position;
        let saved_chars = self.chars.clone();
        let saved_current = self.current_char;

        let token = self.next_token();

        // Restore state
        self.position = saved_position;
        self.chars = saved_chars;
        self.current_char = saved_current;

        token
    }

    fn advance(&mut self) {
        if let Some((_, ch)) = self.current_char {
            self.position.advance(ch);
        }
        self.current_char = self.chars.next();
    }

    fn peek_char(&self) -> Option<char> {
        self.current_char.map(|(_, ch)| ch)
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek_char() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_single_line_comment(&mut self) {
        // Skip the '//' characters
        self.advance(); // first '/'
        self.advance(); // second '/'

        // Skip until end of line or end of file
        while let Some(ch) = self.peek_char() {
            if ch == '\n' {
                self.advance(); // consume the newline
                break;
            }
            self.advance();
        }
    }

    fn skip_multi_line_comment(&mut self) -> Result<(), LexerError> {
        let start_pos = self.position;

        // Skip the '/*' characters
        self.advance(); // first '/'
        self.advance(); // '*'

        while let Some(ch) = self.peek_char() {
            if ch == '*' {
                self.advance(); // consume '*'
                if self.peek_char() == Some('/') {
                    self.advance(); // consume '/'
                    return Ok(());
                }
            } else {
                self.advance();
            }
        }

        // If we reach here, the comment was not terminated
        Err(LexerError::UnterminatedComment {
            line: start_pos.line,
            column: start_pos.column,
        })
    }

    fn skip_whitespace_and_comments(&mut self) -> Result<(), LexerError> {
        loop {
            // Skip whitespace
            self.skip_whitespace();

            // Check for comments
            match self.peek_char() {
                Some('/') => {
                    // Look ahead to see what kind of comment
                    let mut chars_clone = self.chars.clone();
                    if let Some((_, next_ch)) = chars_clone.next() {
                        match next_ch {
                            '/' => {
                                self.skip_single_line_comment();
                                // Continue the loop to check for more whitespace/comments
                                continue;
                            }
                            '*' => {
                                self.skip_multi_line_comment()?;
                                // Continue the loop to check for more whitespace/comments
                                continue;
                            }
                            _ => {
                                // Not a comment, just a division operator
                                break;
                            }
                        }
                    } else {
                        // End of file after '/', not a comment
                        break;
                    }
                }
                _ => break,
            }
        }
        Ok(())
    }

    fn read_string(&mut self, quote: char) -> Result<Token, LexerError> {
        let start_pos = self.position;
        self.advance(); // Skip opening quote

        while let Some(ch) = self.peek_char() {
            if ch == quote {
                self.advance(); // Skip closing quote
                let end_pos = self.position;
                return Ok(Token {
                    token_type: TokenType::String,
                    start: start_pos,
                    end: end_pos,
                });
            }

            if ch == '\\' {
                self.advance(); // Skip escape character
                if self.peek_char().is_some() {
                    self.advance(); // Skip escaped character
                }
            } else {
                self.advance();
            }
        }

        Err(LexerError::UnterminatedString {
            line: start_pos.line,
            column: start_pos.column,
        })
    }

    fn read_number(&mut self) -> Result<Token, LexerError> {
        let start_pos = self.position;

        // Read integer part
        while let Some(ch) = self.peek_char() {
            if ch.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }

        // Check for decimal point
        if self.peek_char() == Some('.') {
            // Look ahead to see if next character is a digit
            if let Some((next_idx, _)) = self.chars.clone().next() {
                if let Some(next_ch) = self.source.chars().nth(next_idx) {
                    if next_ch.is_ascii_digit() {
                        self.advance(); // consume '.'

                        // Read fractional part
                        while let Some(ch) = self.peek_char() {
                            if ch.is_ascii_digit() {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }

        Ok(Token {
            token_type: TokenType::Number,
            start: start_pos,
            end: self.position,
        })
    }

    fn read_identifier(&mut self) -> Token {
        let start_pos = self.position;
        let text_start = self.position.offset;

        while let Some(ch) = self.peek_char() {
            if ch.is_alphanumeric() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let text = &self.source[text_start..self.position.offset];
        let token_type = match text {
            "true" => TokenType::BooleanTrue,
            "false" => TokenType::BooleanFalse,
            "nil" => TokenType::Nil,
            "let" => TokenType::LetKeyword,
            "if" => TokenType::IfKeyword,
            "else" => TokenType::ElseKeyword,
            "while" => TokenType::WhileKeyword,
            "do" => TokenType::DoKeyword,
            "for" => TokenType::ForKeyword,
            "def" => TokenType::DefKeyword,
            "return" => TokenType::ReturnKeyword,
            "class" => TokenType::ClassKeyword,
            "extends" => TokenType::ExtendsKeyword,
            "this" => TokenType::ThisKeyword,
            "super" => TokenType::SuperKeyword,
            "new" => TokenType::NewKeyword,
            "type" => TokenType::TypeKeyword,
            "number" => TokenType::NumberTypeKeyword,
            "string" => TokenType::StringTypeKeyword,
            "boolean" => TokenType::BooleanTypeKeyword,
            "void" => TokenType::VoidTypeKeyword,
            _ => TokenType::Identifier,
        };

        Token {
            token_type,
            start: start_pos,
            end: self.position,
        }
    }
}
