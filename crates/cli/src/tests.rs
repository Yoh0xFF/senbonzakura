use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use super::{Cli, OK, PROBLEMS, RUNTIME_ERROR, USAGE};
use crate::repl::BANNER;

const USAGE_TEXT: &str =
  "usage: senbonzakura [--tree] [--run] <file>\n       senbonzakura --repl\n";

/// What one run of the command line did.
struct Outcome {
  code: i32,
  out: String,
  err: String,
}

/// Runs the command line on a large stack, reading `input` as plain lines.
fn cli(args: &[&str], input: &str) -> Outcome {
  let args: Vec<String> = args.iter().map(|arg| arg.to_string()).collect();
  let input = input.to_string();
  senbonzakura_interpret::with_large_stack(move || {
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let mut input = input.as_bytes();
    let code = Cli::new(&mut input, &mut out, &mut err, false).run(&args);
    Outcome { code, out: String::from_utf8(out).unwrap(), err: String::from_utf8(err).unwrap() }
  })
}

/// A directory of its own for each test, removed when the test ends.
struct TempDir(PathBuf);

impl TempDir {
  fn new() -> TempDir {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let name =
      format!("senbonzakura-cli-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed));
    let dir = std::env::temp_dir().join(name);
    std::fs::create_dir_all(&dir).unwrap();
    TempDir(dir)
  }

  /// Writes a file and returns its path.
  fn write(&self, name: &str, text: &str) -> String {
    let path = self.0.join(name);
    std::fs::write(&path, text).unwrap();
    path.to_string_lossy().into_owned()
  }
}

impl Drop for TempDir {
  fn drop(&mut self) {
    let _ = std::fs::remove_dir_all(&self.0);
  }
}

fn lines_starting_with<'a>(text: &'a str, prefix: &str) -> Vec<&'a str> {
  text.lines().filter(|line| line.starts_with(prefix)).collect()
}

#[test]
fn clean_file_prints_nothing_and_succeeds() {
  let dir = TempDir::new();
  let file = dir.write("ok.sz", "var x: int = 1;\nfn f(a: int): int { return a; }\n");

  let outcome = cli(&[&file], "");

  assert_eq!(outcome.code, OK);
  assert_eq!(outcome.out, "");
  assert_eq!(outcome.err, "");
}

#[test]
fn tree_option_prints_each_top_level_statement() {
  let dir = TempDir::new();
  let file = dir.write("ok.sz", "var x = 1; // one\nfn f() { x++; }\n");

  let outcome = cli(&["--tree", &file], "");

  assert_eq!(outcome.code, OK);
  assert_eq!(outcome.out, "(var x 1)\n(fn f () {(++ x);})\n");
}

#[test]
fn problem_is_printed_with_its_line_and_a_caret() {
  let dir = TempDir::new();
  let file = dir.write("bad.sz", "var x = 1;\nvar y = x +;\n");

  let outcome = cli(&[&file], "");

  assert_eq!(outcome.code, PROBLEMS);
  assert_eq!(
    outcome.err,
    format!(
      "{file}:2:12: error: expected expression, found ';'\n    var y = x +;\n               ^\n"
    )
  );
}

#[test]
fn caret_covers_the_whole_problem() {
  let dir = TempDir::new();
  let file = dir.write("bad.sz", "f(1, x: 2);");

  let outcome = cli(&[&file], "");

  assert_eq!(outcome.code, PROBLEMS);
  assert_eq!(
    outcome.err,
    format!(
      "{file}:1:6: error: positional and named arguments cannot be mixed\n    f(1, x: 2);\n         ^^^^\n"
    )
  );
}

#[test]
fn columns_and_carets_count_characters() {
  let dir = TempDir::new();
  let file = dir.write("bad.sz", "var é = `ü` + 1;");

  let outcome = cli(&[&file], "");

  assert_eq!(
    outcome.err,
    format!(
      "{file}:1:9: error: cannot apply '+' to 'string' and integer constant\n    var é = `ü` + 1;\n            ^^^^^^^\n"
    )
  );
}

#[test]
fn lexer_and_parser_problems_are_printed_in_source_order() {
  let dir = TempDir::new();
  let file = dir.write("bad.sz", "var a = ;\nvar b = '\\q';\nvar c = ;\n");

  let outcome = cli(&[&file], "");

  assert_eq!(outcome.code, PROBLEMS);
  assert_eq!(
    lines_starting_with(&outcome.err, &file),
    [
      format!("{file}:1:9: error: expected expression, found ';'"),
      format!("{file}:2:10: error: unknown escape sequence"),
      format!("{file}:3:9: error: expected expression, found ';'"),
    ]
  );
}

#[test]
fn check_problems_are_printed_like_parse_problems() {
  let dir = TempDir::new();
  let file = dir.write("bad.sz", "fn f(i: int) {}\nf(1, 2);\nbreak;\n");

  let outcome = cli(&[&file], "");

  assert_eq!(outcome.code, PROBLEMS);
  assert_eq!(
    outcome.err,
    format!(
      "{file}:2:1: error: 'f' expects 1 argument, found 2\n    f(1, 2);\n    ^^^^^^^\n\
             {file}:3:1: error: 'break' outside a loop\n    break;\n    ^^^^^^\n"
    )
  );
}

#[test]
fn checks_do_not_run_when_the_file_has_parse_problems() {
  let dir = TempDir::new();
  let file = dir.write("bad.sz", "var = 1;\nx;\n");

  let outcome = cli(&[&file], "");

  assert_eq!(outcome.code, PROBLEMS);
  assert_eq!(
    lines_starting_with(&outcome.err, &file),
    [format!("{file}:1:5: error: expected variable name, found '='")]
  );
}

#[test]
fn tree_is_still_printed_when_there_are_problems() {
  let dir = TempDir::new();
  let file = dir.write("bad.sz", "var x = ;");

  let outcome = cli(&["--tree", &file], "");

  assert_eq!(outcome.code, PROBLEMS);
  assert_eq!(outcome.out, "(var x <error>)\n");
}

#[test]
fn run_option_runs_a_clean_file() {
  let dir = TempDir::new();
  let file = dir.write("ok.sz", "fn square(x: int): int { return x * x; }\nprint(square(7));\n");

  let outcome = cli(&["--run", &file], "");

  assert_eq!(outcome.code, OK);
  assert_eq!(outcome.out, "49\n");
  assert_eq!(outcome.err, "");
}

#[test]
fn run_option_does_not_run_a_file_with_problems() {
  let dir = TempDir::new();
  let file = dir.write("bad.sz", "print(1);\nvar x: int = true;\n");

  let outcome = cli(&["--run", &file], "");

  assert_eq!(outcome.code, PROBLEMS);
  assert_eq!(outcome.out, "");
}

#[test]
fn runtime_error_is_printed_at_the_failing_code() {
  let dir = TempDir::new();
  let file = dir.write("crash.sz", "print(1);\nvar zero = 0;\nprint(10 / zero);\nprint(2);\n");

  let outcome = cli(&["--run", &file], "");

  assert_eq!(outcome.code, RUNTIME_ERROR);
  assert_eq!(outcome.out, "1\n");
  assert_eq!(
    outcome.err,
    format!(
      "{file}:3:7: runtime error: division by zero\n    print(10 / zero);\n          ^^^^^^^^^\n"
    )
  );
}

#[test]
fn runtime_error_without_a_place_names_only_the_file() {
  let dir = TempDir::new();
  let file = dir.write("deep.sz", "fn f(): int { return f() + 1; }\nf();\n");

  let outcome = cli(&["--run", &file], "");

  assert_eq!(outcome.code, RUNTIME_ERROR);
  assert_eq!(
    outcome.err,
    format!("{file}: runtime error: stack overflow: the calls nest too deeply\n")
  );
}

#[test]
fn repl_option_starts_the_prompt_on_the_input() {
  let outcome = cli(&["--repl"], "1 + 2\n");

  assert_eq!(outcome.code, OK);
  assert_eq!(outcome.out, format!("{BANNER}\n> 3\n> \n"));
}

#[test]
fn missing_file_is_a_usage_error() {
  let dir = TempDir::new();
  let file = dir.0.join("missing.sz").to_string_lossy().into_owned();

  let outcome = cli(&[&file], "");

  assert_eq!(outcome.code, USAGE);
  assert_eq!(outcome.err, format!("senbonzakura: cannot read {file}: no such file\n"));
}

#[test]
fn file_that_is_not_utf8_is_a_usage_error() {
  let dir = TempDir::new();
  let path = dir.0.join("latin1.sz");
  std::fs::write(&path, b"var caf\xE9 = 1;").unwrap();
  let file = path.to_string_lossy().into_owned();

  let outcome = cli(&[&file], "");

  assert_eq!(outcome.code, USAGE);
  assert_eq!(outcome.err, format!("senbonzakura: cannot read {file}: not valid UTF-8\n"));
}

#[test]
fn bad_arguments_print_usage() {
  let cases: [&[&str]; 7] = [
    &[],
    &["--tree"],
    &["--run"],
    &["--bogus", "a.sz"],
    &["a.sz", "b.sz"],
    &["--repl", "a.sz"],
    &["--repl", "--run"],
  ];
  for args in cases {
    let outcome = cli(args, "");

    assert_eq!(outcome.code, USAGE, "{args:?}");
    assert_eq!(outcome.err, USAGE_TEXT, "{args:?}");
  }
}
