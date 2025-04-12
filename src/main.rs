use ast::Statement;
use lexer::{Lexer, Token};
use parser::Parser;
use visitor_s_expression::ToSExpression;
use crate::parser::root_statement;

mod ast;
mod lexer;
mod parser;
mod visitor_s_expression;

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

    let mut parser = Parser::new("12; { let x, y = 12; x = 7; let z = x + y; }");
    let ast = root_statement(&mut parser);
    let sexpression = ast.to_pretty_s_expression().unwrap();
    println!("SExpression:\n{}", sexpression);
}
