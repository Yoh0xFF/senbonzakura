use crate::parser::parse_root_statement;
use ast::Statement;
use lexer::{Lexer, Token};
use parser::Parser;
use visitor_s_expression::ToSExpression;

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

    let mut parser = Parser::new(
        r#"
        class Point {
      def constructor(x: number, y: number) {
          this.x = x;
          this.y = y;
      }
  }

  let p: Point = new Point(10, 20);
        "#,
    );
    let ast = parse_root_statement(&mut parser);
    let s_expression = ast.to_pretty_s_expression().unwrap();
    println!("SExpression:\n{}", s_expression);

    println!("Yaml:\n{}", serde_yaml::to_string(&ast).unwrap());
}
