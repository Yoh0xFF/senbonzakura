use lexer::{Lexer, Token};
use parser::{Parser, Statement};

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
    let statement = parser.parse();

    match statement.as_ref() {
        Statement::Program { body } => {
            println!("Program {:?}", body);
        }
        Statement::BlockStatement { body } => {
            println!("BlockStatement {:?}", body);
        }
        Statement::EmptyStatement => {
            println!("EmptyStatement");
        }
        Statement::ExpressionStatement { expression } => {
            println!("ExpressionStatement {:?}", expression);
        }
    }
}
