//! Where the [`Repl`](crate::repl::Repl) reads its entries from: a plain stream, or a terminal
//! with line editing.

use std::borrow::Cow;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;

use nu_ansi_term::Color;
use reedline::{
  FileBackedHistory, Prompt, PromptEditMode, PromptHistorySearch, Reedline, Signal,
  ValidationResult, Validator,
};

use crate::repl::{MORE, PROMPT, unfinished};

pub trait ReplInput {
  /// Reads the next entry, which may span lines, or returns `None` when the input has ended. An
  /// entry the user abandoned comes back blank. Prompts, if the input prints its own, go to
  /// `out`.
  fn next(&mut self, out: &mut dyn Write) -> io::Result<Option<String>>;
}

/// Reads lines from a stream, printing the prompts itself, and joins the lines of an entry that is
/// still [`unfinished`]. A blank line ends an entry anyway.
pub struct Plain<R> {
  input: R,
}

impl<R: BufRead> Plain<R> {
  pub fn new(input: R) -> Plain<R> {
    Plain { input }
  }

  /// Reads one line without its line break, or `None` at the end of the input. Bytes that are
  /// not UTF-8 become U+FFFD.
  fn read_line(&mut self) -> io::Result<Option<String>> {
    let mut bytes = Vec::new();
    if self.input.read_until(b'\n', &mut bytes)? == 0 {
      return Ok(None);
    }
    if bytes.last() == Some(&b'\n') {
      bytes.pop();
      if bytes.last() == Some(&b'\r') {
        bytes.pop();
      }
    }
    Ok(Some(String::from_utf8_lossy(&bytes).into_owned()))
  }
}

impl<R: BufRead> ReplInput for Plain<R> {
  fn next(&mut self, out: &mut dyn Write) -> io::Result<Option<String>> {
    write!(out, "{PROMPT}")?;
    out.flush()?;
    let mut text = String::new();
    loop {
      let Some(line) = self.read_line()? else {
        return Ok(if text.is_empty() { None } else { Some(text) });
      };
      if !text.is_empty() && line.trim().is_empty() {
        return Ok(Some(text));
      }
      text.push_str(&line);
      text.push('\n');
      if !unfinished(&text) {
        return Ok(Some(text));
      }
      write!(out, "{MORE}")?;
      out.flush()?;
    }
  }
}

/// Reads from a terminal with line editing: the arrow keys move through the entry, which can be
/// edited anywhere, and up and down recall earlier entries. Enter on an unfinished entry starts a
/// new line of it instead of submitting it, so a whole function stays one entry to edit and
/// recall. Ctrl-C abandons the entry, and Ctrl-D on an empty one ends the input.
pub struct Terminal {
  editor: Reedline,
}

impl Terminal {
  /// `history` is the file earlier entries are kept in across sessions, or `None` to keep them
  /// only while this session lasts.
  pub fn new(history: Option<PathBuf>) -> io::Result<Terminal> {
    let mut editor = Reedline::create().with_validator(Box::new(EntryValidator));
    if let Some(path) = history {
      let history = FileBackedHistory::with_file(1000, path).map_err(io::Error::other)?;
      editor = editor.with_history(Box::new(history));
    }
    Ok(Terminal { editor })
  }
}

impl ReplInput for Terminal {
  fn next(&mut self, out: &mut dyn Write) -> io::Result<Option<String>> {
    out.flush()?;
    match self.editor.read_line(&EntryPrompt)? {
      Signal::Success(entry) => Ok(Some(entry)),
      Signal::CtrlC(_) => Ok(Some(String::new())),
      Signal::CtrlD => Ok(None),
      _ => Ok(Some(String::new())),
    }
  }
}

/// Tells the line editor whether Enter submits the entry. It does unless the entry is
/// [`unfinished`], and a blank last line submits it anyway.
pub(crate) struct EntryValidator;

impl Validator for EntryValidator {
  fn validate(&self, line: &str) -> ValidationResult {
    if unfinished(line) && !ends_blank(line) {
      ValidationResult::Incomplete
    } else {
      ValidationResult::Complete
    }
  }
}

fn ends_blank(line: &str) -> bool {
  line.rfind('\n').is_some_and(|last_break| line[last_break + 1..].trim().is_empty())
}

/// `> ` before an entry and `... ` before each further line of it, in the terminal's own color.
struct EntryPrompt;

impl Prompt for EntryPrompt {
  fn render_prompt_left(&self) -> Cow<'_, str> {
    Cow::Borrowed(PROMPT)
  }

  fn render_prompt_right(&self) -> Cow<'_, str> {
    Cow::Borrowed("")
  }

  fn render_prompt_indicator(&self, _mode: PromptEditMode) -> Cow<'_, str> {
    Cow::Borrowed("")
  }

  fn render_prompt_multiline_indicator(&self) -> Cow<'_, str> {
    Cow::Borrowed(MORE)
  }

  fn render_prompt_history_search_indicator(&self, search: PromptHistorySearch) -> Cow<'_, str> {
    Cow::Owned(format!("(search: {}) ", search.term))
  }

  fn get_prompt_color(&self) -> Color {
    Color::Default
  }

  fn get_prompt_multiline_color(&self) -> Color {
    Color::Default
  }

  fn get_indicator_color(&self) -> Color {
    Color::Default
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn enter_on_an_unfinished_entry_starts_a_new_line() {
    for entry in ["fn f() {", "f(1,", "fn f() {\n  return 1;", "`multi\nline", "/* open"] {
      assert!(matches!(EntryValidator.validate(entry), ValidationResult::Incomplete), "{entry:?}");
    }
  }

  #[test]
  fn enter_on_a_finished_or_blank_ended_entry_submits_it() {
    let entries = [
      "1 + 2",
      "fn f() {\n  return 1;\n}",
      "// don't stop at an apostrophe",
      "var s = \"it's\"; var c = '\\'';",
      "print(`a \\ b`)",
      "{\n",
      "fn f() {\n   ",
    ];
    for entry in entries {
      assert!(matches!(EntryValidator.validate(entry), ValidationResult::Complete), "{entry:?}");
    }
  }

  #[test]
  fn plain_input_reads_lines_until_the_entry_is_finished() {
    let mut out = Vec::new();
    let mut input = Plain::new("fn f() {\n}\r\n1\n".as_bytes());

    assert_eq!(input.next(&mut out).unwrap().as_deref(), Some("fn f() {\n}\n"));
    assert_eq!(input.next(&mut out).unwrap().as_deref(), Some("1\n"));
    assert_eq!(input.next(&mut out).unwrap(), None);
    assert_eq!(String::from_utf8(out).unwrap(), "> ... > > ");
  }
}
