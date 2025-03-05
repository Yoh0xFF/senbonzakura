use lexer::{Lexer, Token};
use parser::{Expression, Parser};

mod lexer;
mod parser;

fn main() {
    let mut lexer = Lexer::new(r#"12 17 "Hello"   'world' "#);
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

    let mut parser = Parser::new("12;");
    let expression = parser.parse();

    match expression.as_ref() {
        Expression::Program { body } => {
            println!("Program {:?}", body);
        }
        Expression::ExpressionStatement { expression } => {
            println!("ExpressionStatement {:?}", expression);
        }
        Expression::NumericLiteral(val) => {
            println!("NumericLiteral {:?}", val);
        }
        Expression::StringLiteral(val) => {
            println!("StringLiteral {:?}", val);
        }
    }
}
