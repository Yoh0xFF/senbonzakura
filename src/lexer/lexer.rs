use super::{
    token::{Token, TokenType},
    LexerError, LexerResult, TokenPosition,
};

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    source: &'a str,
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

    pub fn next_token(&mut self) -> LexerResult<Token> {
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

            // Single character tokens
            ';' => self.consume_single_char_token(TokenType::StatementEnd, start_pos),
            ':' => self.consume_single_char_token(TokenType::Colon, start_pos),
            '.' => self.consume_single_char_token(TokenType::Dot, start_pos),
            ',' => self.consume_single_char_token(TokenType::Comma, start_pos),
            '{' => self.consume_single_char_token(TokenType::OpeningBrace, start_pos),
            '}' => self.consume_single_char_token(TokenType::ClosingBrace, start_pos),
            '(' => self.consume_single_char_token(TokenType::OpeningParenthesis, start_pos),
            ')' => self.consume_single_char_token(TokenType::ClosingParenthesis, start_pos),
            '[' => self.consume_single_char_token(TokenType::OpeningBracket, start_pos),
            ']' => self.consume_single_char_token(TokenType::ClosingBracket, start_pos),

            // Multi-character operators
            '=' => self.read_equality_or_assignment(start_pos),
            '+' => self.read_plus_operator(start_pos),
            '-' => self.read_minus_operator(start_pos),
            '*' => self.read_multiply_operator(start_pos),
            '/' => self.read_divide_operator(start_pos),
            '>' => self.read_greater_than_operator(start_pos),
            '<' => self.read_less_than_operator(start_pos),
            '&' => self.read_logical_and(start_pos),
            '|' => self.read_logical_or(start_pos),
            '!' => self.read_not_operator(start_pos),

            _ => Err(LexerError::UnexpectedCharacter {
                char: ch,
                line: start_pos.line,
                column: start_pos.column,
            }),
        }
    }

    #[allow(dead_code)]
    pub fn peek_token(&mut self) -> LexerResult<Token> {
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

    // Helper method for single character tokens
    fn consume_single_char_token(
        &mut self,
        token_type: TokenType,
        start_pos: TokenPosition,
    ) -> LexerResult<Token> {
        self.advance();
        Ok(Token {
            token_type,
            start: start_pos,
            end: self.position,
        })
    }

    // Optimized operator reading methods
    fn read_equality_or_assignment(&mut self, start_pos: TokenPosition) -> LexerResult<Token> {
        self.advance(); // consume '='

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

    fn read_plus_operator(&mut self, start_pos: TokenPosition) -> LexerResult<Token> {
        self.advance(); // consume '+'

        let token_type = if self.peek_char() == Some('=') {
            self.advance();
            TokenType::ComplexPlusAssignmentOperator
        } else {
            TokenType::AdditivePlusOperator
        };

        Ok(Token {
            token_type,
            start: start_pos,
            end: self.position,
        })
    }

    fn read_minus_operator(&mut self, start_pos: TokenPosition) -> LexerResult<Token> {
        self.advance(); // consume '-'

        let token_type = if self.peek_char() == Some('=') {
            self.advance();
            TokenType::ComplexMinusAssignmentOperator
        } else {
            TokenType::AdditiveMinusOperator
        };

        Ok(Token {
            token_type,
            start: start_pos,
            end: self.position,
        })
    }

    fn read_multiply_operator(&mut self, start_pos: TokenPosition) -> LexerResult<Token> {
        self.advance(); // consume '*'

        let token_type = if self.peek_char() == Some('=') {
            self.advance();
            TokenType::ComplexMultiplyAssignmentOperator
        } else {
            TokenType::FactorMultiplicationOperator
        };

        Ok(Token {
            token_type,
            start: start_pos,
            end: self.position,
        })
    }

    fn read_divide_operator(&mut self, start_pos: TokenPosition) -> LexerResult<Token> {
        self.advance(); // consume '/'

        let token_type = if self.peek_char() == Some('=') {
            self.advance();
            TokenType::ComplexDivideAssignmentOperator
        } else {
            TokenType::FactorDivisionOperator
        };

        Ok(Token {
            token_type,
            start: start_pos,
            end: self.position,
        })
    }

    fn read_greater_than_operator(&mut self, start_pos: TokenPosition) -> LexerResult<Token> {
        self.advance(); // consume '>'

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

    fn read_less_than_operator(&mut self, start_pos: TokenPosition) -> LexerResult<Token> {
        self.advance(); // consume '<'

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

    fn read_logical_and(&mut self, start_pos: TokenPosition) -> LexerResult<Token> {
        self.advance(); // consume first '&'

        if self.peek_char() == Some('&') {
            self.advance();
            Ok(Token {
                token_type: TokenType::LogicalAndOperator,
                start: start_pos,
                end: self.position,
            })
        } else {
            Err(LexerError::UnexpectedCharacter {
                char: '&',
                line: start_pos.line,
                column: start_pos.column,
            })
        }
    }

    fn read_logical_or(&mut self, start_pos: TokenPosition) -> LexerResult<Token> {
        self.advance(); // consume first '|'

        if self.peek_char() == Some('|') {
            self.advance();
            Ok(Token {
                token_type: TokenType::LogicalOrOperator,
                start: start_pos,
                end: self.position,
            })
        } else {
            Err(LexerError::UnexpectedCharacter {
                char: '|',
                line: start_pos.line,
                column: start_pos.column,
            })
        }
    }

    fn read_not_operator(&mut self, start_pos: TokenPosition) -> LexerResult<Token> {
        self.advance(); // consume '!'

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

    // Improved string reading with escape sequence support
    fn read_string(&mut self, quote: char) -> LexerResult<Token> {
        let start_pos = self.position;
        self.advance(); // Skip opening quote

        while let Some(ch) = self.peek_char() {
            if ch == quote {
                self.advance(); // Skip closing quote
                return Ok(Token {
                    token_type: TokenType::String,
                    start: start_pos,
                    end: self.position,
                });
            }

            if ch == '\\' {
                self.advance(); // Skip escape character
                if let Some(escaped_char) = self.peek_char() {
                    // Validate common escape sequences
                    match escaped_char {
                        'n' | 't' | 'r' | '\\' | '\'' | '"' | '0' => {
                            self.advance(); // Valid escape sequence
                        }
                        'x' => {
                            // Hex escape sequence \xNN
                            self.advance();
                            if self.peek_char().map_or(false, |c| c.is_ascii_hexdigit()) {
                                self.advance();
                                if self.peek_char().map_or(false, |c| c.is_ascii_hexdigit()) {
                                    self.advance();
                                }
                            }
                        }
                        _ => {
                            // Unknown escape sequence, but continue parsing
                            self.advance();
                        }
                    }
                } else {
                    // Escape at end of file
                    break;
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

    // Enhanced number reading with better float support
    fn read_number(&mut self) -> LexerResult<Token> {
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
            // Look ahead to ensure next character is a digit (not a method call)
            let mut chars_clone = self.chars.clone();
            if let Some((_, next_ch)) = chars_clone.next() {
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

        // Check for scientific notation (e.g., 1.5e-10, 3E+4)
        if let Some(ch) = self.peek_char() {
            if ch == 'e' || ch == 'E' {
                self.advance(); // consume 'e' or 'E'

                // Optional sign
                if let Some(sign_ch) = self.peek_char() {
                    if sign_ch == '+' || sign_ch == '-' {
                        self.advance();
                    }
                }

                // Exponent digits (required)
                let mut has_exponent_digits = false;
                while let Some(ch) = self.peek_char() {
                    if ch.is_ascii_digit() {
                        self.advance();
                        has_exponent_digits = true;
                    } else {
                        break;
                    }
                }

                if !has_exponent_digits {
                    return Err(LexerError::InvalidNumber {
                        line: start_pos.line,
                        column: start_pos.column,
                    });
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
        let token_type = Self::classify_keyword_or_identifier(text);

        Token {
            token_type,
            start: start_pos,
            end: self.position,
        }
    }

    // Separate method for keyword classification (easier to maintain)
    fn classify_keyword_or_identifier(text: &str) -> TokenType {
        match text {
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
        }
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
            while let Some(ch) = self.peek_char() {
                if ch.is_whitespace() {
                    self.advance();
                } else {
                    break;
                }
            }

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
}
