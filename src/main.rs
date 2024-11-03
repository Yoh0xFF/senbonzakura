use lexer::{Lexer, Token};

mod lexer;

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
}
