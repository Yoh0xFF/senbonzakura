use lexer::{Lexer, Token};
use parser::{Expression, Parser};

mod lexer;
mod parser;

fn main() {
    let lexer = Lexer::new(r#"12 17 "Hello"   'world' "#);
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

    let parser = Parser::new("12");
    let expression = parser.parse();

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
