use std::cell::RefCell;

use super::Expression;

/**
 * Senbonzakura recursive descent parser
 */
#[derive(Debug, Clone)]
pub struct Parser {
    source: RefCell<String>,
}

impl Parser {
    /**
     * Parses a string into an AST
     */
    pub fn new() -> Self {
        Parser {
            source: RefCell::new(String::from("")),
        }
    }

    pub fn parse(&self, source: &str) -> Expression {
        self.source.replace(source.to_string());
        self.program()
    }

    /**
     * Main entry point
     *
     * Program
     *  : NumericLiteral
     *  ;
     */
    fn program(&self) -> Expression {
        self.numeric_literal()
    }

    /**
     * NumericLiteral
     *  : NUMBER
     *  ;
     */
    fn numeric_literal(&self) -> Expression {
        let source = self.source.borrow();
        let number: i32 = source.parse().unwrap();
        Expression::NumericLiteral(number)
    }
}
