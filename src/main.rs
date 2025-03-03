use lexer::{Lexer, Token};
use parser::{Expression, Parser};

mod lexer;
mod parser;

fn main() {
    let source = r#"12 17 "Hello"   'world' "#;

    let lexer = Lexer::new(String::from(source));
    let next_token: Token = lexer.next_token();
    println!("Token: {}", next_token);
    let next_token = lexer.next_token();
    println!("Token: {}", next_token);
    let next_token = lexer.next_token();
    println!("Token: {}", next_token);
    let next_token = lexer.next_token();
    println!("Token: {}", next_token);
    let next_token = lexer.next_token();
    println!("Token: {}", next_token);

    let parser = Parser::new();
    let expression = parser.parse("12");

    match expression {
        Expression::NumericLiteral(x) => {
            println!("NumericLiteral {}", x);
        }
    }
}
