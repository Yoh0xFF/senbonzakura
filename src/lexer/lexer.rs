use std::cell::Cell;

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
    index: Cell<usize>,
    rules: Vec<(Regex, TokenType)>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let rules = REGEX_RULES.get_or_init(|| init_regex_rules()).to_vec();

        Lexer {
            source,
            index: Cell::new(0),
            rules,
        }
    }

    /**
     * Obtain next token
     */
    pub fn next_token(&self) -> Token {
        let crnt_index = self.index.get();

        // Check if we're at the end of the source
        if crnt_index >= self.source.len() {
            return Token {
                token_type: TokenType::End,
                i: crnt_index,
                j: crnt_index,
            };
        }

        // Slice the string starting from the current position
        let expression = &self.source[crnt_index..];

        for (regex, token_type) in &self.rules {
            let token: &str;
            match regex.find(expression) {
                None => continue, // Try to match other token
                Some(x) => token = x.as_str(),
            }
            let token_len = token.chars().count();

            // Skip whitespace
            if *token_type == TokenType::Whitespace {
                self.index.set(crnt_index + token_len);
                return self.next_token();
            }

            self.index.set(crnt_index + token_len);
            return Token {
                token_type: *token_type,
                i: crnt_index,
                j: crnt_index + token_len,
            };
        }

        // If we get here, no token matched
        panic!(
            "Invalid token at index {}, remaining text: '{}'",
            crnt_index,
            &self.source[crnt_index..]
        );
    }
}
