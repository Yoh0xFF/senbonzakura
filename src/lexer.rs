#[derive(Debug, Clone)]
struct Lexer {
    index: u32,
}

impl Lexer {
    pub fn new() -> Self {
        Lexer { index: 0 }
    }
}
