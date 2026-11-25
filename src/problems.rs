//! Prints problems the way the command line and the REPL both show them.

use std::io::Write;

use crate::lexer::{SourceFile, char_count};

/// Prints `path:line:column: kind: message`, then the line and a caret under the part the problem
/// covers. A source with an empty path, like a REPL entry, starts at the line instead.
pub(crate) fn print(
    err: &mut dyn Write,
    source: &SourceFile,
    start: usize,
    end: usize,
    kind: &str,
    message: &str,
) {
    let at = source.locate(start);
    let place =
        if source.path().is_empty() { at.to_string() } else { format!("{}:{at}", source.path()) };
    let line = source.line_text(at.line);
    let length = char_count(&source.text().as_bytes()[start..end]);
    // Like the program's own output, a closed error stream does not stop the program.
    let _ = writeln!(err, "{place}: {kind}: {message}");
    let _ = writeln!(err, "    {line}");
    let _ = writeln!(err, "    {}", caret(line, at.column, length));
}

/// Pads up to `column` and marks `length` characters with `^`, at least one and no further than
/// the end of the line. The padding keeps the line's tabs, so the caret lines up under
/// tab-indented code.
pub(crate) fn caret(line: &str, column: usize, length: usize) -> String {
    let from = column - 1;
    let mut caret: String =
        line.chars().take(from).map(|c| if c == '\t' { '\t' } else { ' ' }).collect();
    let room = line.chars().count().saturating_sub(from);
    caret.push_str(&"^".repeat(length.min(room).max(1)));
    caret
}

#[cfg(test)]
mod tests {
    use super::caret;

    #[test]
    fn caret_lines_up_under_the_line() {
        let cases = [
            ("abc", 2, 1, " ^"),
            ("abc", 1, 3, "^^^"),
            ("abc", 2, 9, " ^^"),
            ("abc", 4, 0, "   ^"),
            ("\tx = ;", 6, 1, "\t    ^"),
            ("é = ;", 5, 1, "    ^"),
        ];
        for (line, column, length, expected) in cases {
            assert_eq!(caret(line, column, length), expected, "{line:?} {column} {length}");
        }
    }
}
