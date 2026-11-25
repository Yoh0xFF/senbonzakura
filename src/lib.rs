//! Senbonzakura: a small statically typed language.
//!
//! Source text goes through the [`lexer`] into tokens, the [`parser`] into an [`ast`], the
//! [`check`]er for names and types, and finally the [`interpret`]er. The [`cli`] drives the
//! pipeline for a file, and the [`repl`] for one entry at a time.

pub mod ast;
pub mod check;
pub mod cli;
mod float_text;
pub mod interpret;
pub mod lexer;
pub mod parser;
mod problems;
pub mod repl;
mod repl_input;

/// Runs `f` on a thread with a stack large enough for the interpreter's deepest allowed
/// recursion, and returns what it returns. Rust cannot catch a stack overflow, so the
/// interpreter limits how deeply calls nest, and this gives that limit room to be reached.
pub fn with_large_stack<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> T {
    const STACK_SIZE: usize = 1 << 30;
    std::thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(f)
        .expect("cannot start the interpreter thread")
        .join()
        .unwrap_or_else(|panic| std::panic::resume_unwind(panic))
}
