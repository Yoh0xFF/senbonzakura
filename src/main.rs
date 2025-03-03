use lexer::{Lexer, Token};
use parser::{Expression, Parser};

mod lexer;
mod parser;

fn main() {
    let mut lexer = Lexer::new();
    lexer.init(r#"12 17 "Hello"   'world' "#);
    let next_token = lexer.next_token();
    println!("Token: {}", next_token);
    let next_token = lexer.next_token();
    println!("Token: {}", next_token);
    let next_token = lexer.next_token();
    println!("Token: {}", next_token);
    let next_token = lexer.next_token();
    println!("Token: {}", next_token);
    let next_token = lexer.next_token();
    println!("Token: {}", next_token);

    let mut parser = Parser::new();
    let expression = parser.parse("12");

    match expression {
        Expression::Program { body } => {
            println!("Program {:?}", body);
        }
        Expression::NumericLiteral(val) => {
            println!("NumericLiteral {:?}", val);
        }
        Expression::StringLiteral(val) => {
            println!("StringLiteral {:?}", val);
        }
    }
}
