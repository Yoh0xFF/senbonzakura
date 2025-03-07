use ast::Statement;
use lexer::{Lexer, Token};
use parser::Parser;

mod ast;
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
        Statement::Block { body } => {
            println!("BlockStatement {:?}", body);
        }
        Statement::Empty => {
            println!("EmptyStatement");
        }
        Statement::Expression { expression } => {
            println!("ExpressionStatement {:?}", expression);
        }
    }
}
