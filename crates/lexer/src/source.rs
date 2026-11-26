use std::fmt;

use super::Token;

/// The text being lexed, with its path and the line starts that [`locate`](Self::locate) uses.
#[derive(Debug, Clone)]
pub struct SourceFile {
    path: String,
    text: String,
    line_starts: Vec<usize>,
}

impl SourceFile {
    pub fn new(path: impl Into<String>, text: impl Into<String>) -> SourceFile {
        let text = text.into();
        let line_starts = line_starts(&text);
        SourceFile { path: path.into(), text, line_starts }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    /// The length of the text in bytes.
    pub fn len(&self) -> usize {
        self.text.len()
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    /// The number of lines. Text that ends in a line break has an empty last line.
    pub fn line_count(&self) -> usize {
        self.line_starts.len()
    }

    pub fn text_of(&self, token: &Token) -> &str {
        &self.text[token.start..token.end]
    }

    /// Returns the text of a 1-based line, without its line break.
    ///
    /// # Panics
    ///
    /// If `line` is outside `1` to [`line_count`](Self::line_count).
    pub fn line_text(&self, line: usize) -> &str {
        assert!(
            (1..=self.line_starts.len()).contains(&line),
            "line {line} outside 1..{} in {}",
            self.line_starts.len(),
            self.path
        );
        let start = self.line_starts[line - 1];
        let end = self.line_starts.get(line).copied().unwrap_or(self.text.len());
        self.text[start..end].trim_end_matches('\n').trim_end_matches('\r')
    }

    /// Converts a byte offset into a 1-based line and column. Columns count characters, as rustc
    /// does, and `\r\n` counts as one line break.
    ///
    /// # Panics
    ///
    /// If `offset` is outside `0` to [`len`](Self::len), inclusive.
    pub fn locate(&self, offset: usize) -> LineCol {
        assert!(
            offset <= self.text.len(),
            "offset {offset} outside 0..{} in {}",
            self.text.len(),
            self.path
        );
        // The first line start past offset, minus one, is the line holding it.
        let line = self.line_starts.partition_point(|&start| start <= offset) - 1;
        let column = char_count(&self.text.as_bytes()[self.line_starts[line]..offset]);
        LineCol::new(line + 1, column + 1)
    }
}

impl fmt::Display for SourceFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.path)
    }
}

/// Counts the characters in UTF-8 bytes, which need not start or end on a character boundary.
pub fn char_count(bytes: &[u8]) -> usize {
    bytes.iter().filter(|&&b| b & 0xC0 != 0x80).count()
}

fn line_starts(text: &str) -> Vec<usize> {
    let bytes = text.as_bytes();
    let mut starts = vec![0];
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'\r' if bytes.get(i + 1) == Some(&b'\n') => {
                i += 1;
                starts.push(i + 1);
            }
            b'\n' | b'\r' => starts.push(i + 1),
            _ => {}
        }
        i += 1;
    }
    starts
}

/// A 1-based line and column, printed as `line:column`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineCol {
    pub line: usize,
    pub column: usize,
}

impl LineCol {
    /// # Panics
    ///
    /// If either is 0.
    pub fn new(line: usize, column: usize) -> LineCol {
        assert!(line >= 1 && column >= 1, "line and column are 1-based: {line}:{column}");
        LineCol { line, column }
    }
}

impl fmt::Display for LineCol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}
