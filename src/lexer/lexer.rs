use std::cell::Cell;

use regex::Regex;

use super::{
    regex_rules::{init_regex_rules, REGEX_RULES},
    token::{Token, TokenType},
};

#[derive(Debug, Clone)]
pub struct Lexer {
    source: String,
    index: Cell<usize>,
    rules: Vec<(Regex, TokenType)>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        let rules = REGEX_RULES.get_or_init(|| init_regex_rules()).to_vec();

        Lexer {
            source,
            index: Cell::new(0),
            rules,
        }
    }

    pub fn next_token(&self) -> Token {
        let crnt_index = self.index();
        let expression = &self.source[crnt_index..];

        for (regex, token_type) in &self.rules {
            let token: &str;
            match regex.find(expression) {
                None => continue, // Try to match other token
                Some(x) => token = x.as_str(),
            }

            // Skip whitespace
            if *token_type == TokenType::Whitespace {
                self.increment_by_n(token.chars().count());
                return self.next_token();
            }

            self.increment_by_n(token.chars().count());
            let token = Token {
                index: crnt_index,
                token_type: *token_type,
                value: String::from(token),
            };
            return token;
        }

        let updated_index = self.index();
        if updated_index == self.source.chars().count() {
            let token = Token {
                index: crnt_index,
                token_type: TokenType::End,
                value: String::from("."),
            };
            return token;
        } else {
            panic!("Invalid expression at index {}", updated_index);
        }
    }

    fn increment_by_n(&self, n: usize) {
        self.index.set(self.index.get() + n);
    }

    fn index(&self) -> usize {
        return self.index.get();
    }
}
