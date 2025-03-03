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
pub struct Lexer {
    source: String,
    index: usize,
    rules: Vec<(Regex, TokenType)>,
}

impl Lexer {
    pub fn new() -> Self {
        let rules = REGEX_RULES.get_or_init(|| init_regex_rules()).to_vec();

        Lexer {
            source: "".to_string(),
            index: 0,
            rules,
        }
    }

    /**
     * Initialize the string
     */
    pub fn init(&mut self, source: &str) {
        self.source = source.to_string();
        self.index = 0;
    }

    /**
     * Obtain next token
     */
    pub fn next_token(&mut self) -> Token {
        let crnt_index = self.index;

        // Check if we're at the end of the source
        if crnt_index >= self.source.len() {
            return Token {
                index: crnt_index,
                token_type: TokenType::End,
                value: ".".to_string(),
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
                self.index = self.index + token_len;
                return self.next_token();
            }

            self.index = self.index + token_len;

            let value = match token_type {
                TokenType::String => String::from(&token[1..token_len - 1]),
                _ => String::from(token),
            };

            let token = Token {
                index: crnt_index,
                token_type: *token_type,
                value,
            };
            return token;
        }

        // If we get here, no token matched
        panic!(
            "Invalid token at index {}, remaining text: '{}'",
            crnt_index,
            &self.source[crnt_index..]
        );
    }
}
