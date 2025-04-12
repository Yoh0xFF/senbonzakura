use regex::Regex;

use super::{
    regex_rules::{init_regex_rules, REGEX_RULES},
    token::{Token, TokenType},
};

/**
 * Lexer class
 *
 * Lazily pulls a token from a stream
 */
#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    source: &'a str,
    index: usize,
    rules: Vec<(Regex, TokenType)>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let rules = REGEX_RULES.get_or_init(|| init_regex_rules()).to_vec();

        Lexer {
            source,
            index: 0,
            rules,
        }
    }

    /**
     * Obtain next token
     */
    pub fn next_token(&mut self) -> Token {
        let current_index = self.index;

        // Check if we're at the end of the source
        if current_index >= self.source.len() {
            return Token {
                token_type: TokenType::End,
                i: current_index,
                j: current_index,
            };
        }

        // Slice the string starting from the current position
        let expression = &self.source[current_index..];

        for (regex, token_type) in &self.rules {
            let token: &str;
            match regex.find(expression) {
                None => continue,              // Try to match other token
                Some(x) => token = x.as_str(), // Return reference to the matched token
            }
            let token_len = token.chars().count();

            return match *token_type {
                TokenType::Whitespace
                | TokenType::SingleLineComment
                | TokenType::MultiLineComment => {
                    // Skip whitespace and comments
                    self.index = current_index + token_len;
                    self.next_token()
                }
                _ => {
                    // Return the matched token
                    self.index = current_index + token_len;
                    Token {
                        token_type: *token_type,
                        i: current_index,
                        j: current_index + token_len,
                    }
                }
            };
        }

        // If we get here, no token matched
        panic!(
            "Invalid token at index {}, remaining text: '{}'",
            current_index,
            &self.source[current_index..]
        );
    }
}
