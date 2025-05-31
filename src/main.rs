// use crate::parser::parse_root_statement;
use lexer::{Lexer, Token, TokenType};
use parser::Parser;

mod ast;
mod lexer;
mod parser;

fn main() {
    /*
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
    println!("Yaml:\n{}", serde_yaml::to_string(&ast).unwrap());
    */
    let source = r#"let sum = 0;
for (let i = 0; i < 10; i += 1) {
  for (let j = 0; j < 10; j += 1) {
    sum += i * j;
  }
}
"#;
    let mut lexer = Lexer::new(&source);

    let mut actual_tokens: Vec<Token> = vec![];
    let mut token = match lexer.next_token() {
        Ok(token) => token,
        Err(error) => panic!("Failed to get next token: {:?}", error),
    };
    while token.token_type != TokenType::End {
        actual_tokens.push(token);
        token = match lexer.next_token() {
            Ok(token) => token,
            Err(error) => panic!("Failed to get next token: {:?}", error),
        };
    }

    println!("Yaml:\n{}", serde_yaml::to_string(&actual_tokens).unwrap());
}
