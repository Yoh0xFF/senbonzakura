use super::{BANNER, MORE, PROMPT, Plain, Repl, unfinished};
use crate::cli::OK;

/// Runs the REPL on the input, on a large stack, and returns its exit code, output and errors.
fn session(input: &str) -> (i32, String, String) {
    let input = input.to_string();
    crate::with_large_stack(move || {
        let (mut out, mut err) = (Vec::new(), Vec::new());
        let code = Repl::new(Box::new(Plain::new(input.as_bytes())), &mut out, &mut err).run();
        (code, String::from_utf8(out).unwrap(), String::from_utf8(err).unwrap())
    })
}

/// What the REPL showed for the input, without the banner and prompts, and its errors.
fn values(input: &str) -> (Vec<String>, String) {
    let (_, out, err) = session(input);
    let shown = &out[BANNER.len() + 1..];
    let values = shown
        .replace(MORE, "")
        .replace(PROMPT, "")
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect();
    (values, err)
}

#[test]
fn expression_without_semicolon_shows_its_value() {
    assert_eq!(values("1 + 2\n").0, ["3"]);
}

#[test]
fn entry_with_semicolon_shows_nothing() {
    assert_eq!(values("1 + 2;\nvar x = 1;\n").0, Vec::<String>::new());
}

#[test]
fn variables_and_functions_last_between_entries() {
    let (values, _) = values("var x = 2;\nx * 21\nfn sq(n: int): int { return n * n; }\nsq(9)\n");

    assert_eq!(values, ["42", "81"]);
}

#[test]
fn last_expression_of_an_entry_is_shown() {
    assert_eq!(values("var a = 2; var b = 3; a * b\n").0, ["6"]);
}

#[test]
fn print_is_not_shown_twice() {
    assert_eq!(values("print(`hi`)\n").0, ["hi"]);
}

#[test]
fn values_without_a_printed_form_are_not_shown() {
    assert_eq!(values("fn pair(): int, int { return 1, 2; }\npair()\n").0, Vec::<String>::new());
}

#[test]
fn declaring_a_name_again_shadows_it() {
    assert_eq!(values("var x = 1;\nvar x = `s`;\nx\n").0, ["s"]);
}

#[test]
fn entry_continues_while_a_brace_is_open() {
    let (_, out, _) = session("fn f(): int {\n  return 5;\n}\nf()\n");

    assert_eq!(values("fn f(): int {\n  return 5;\n}\nf()\n").0, ["5"]);
    assert!(out.contains(MORE), "{out}");
}

#[test]
fn entry_continues_while_a_string_or_comment_is_open() {
    assert_eq!(values("`a\nb`\n/* a\ncomment */ 5\n").0, ["a", "b", "5"]);
}

#[test]
fn blank_line_ends_an_unfinished_entry() {
    let (values, err) = values("{\n\n1\n");

    assert_eq!(values, ["1"]);
    assert!(err.contains("2:1: error: expected '}', found end of input"), "{err}");
}

#[test]
fn problem_is_reported_and_the_entry_forgotten() {
    let (values, err) = values("var y: int = true;\ny\n");

    assert_eq!(values, Vec::<String>::new());
    assert_eq!(
        err,
        "1:14: error: expected 'int', found 'bool'\n    var y: int = true;\n                 ^^^^\n\
         1:1: error: 'y' is not defined\n    y\n    ^\n"
    );
}

#[test]
fn runtime_error_is_reported_and_the_entry_forgotten() {
    let (values, err) = values("var z = 1 / 0;\nz\n");

    assert_eq!(values, Vec::<String>::new());
    assert!(err.starts_with("1:9: runtime error: division by zero\n"), "{err}");
    assert!(err.contains("'z' is not defined"), "{err}");
}

#[test]
fn failed_entry_leaves_earlier_variables_alone() {
    assert_eq!(values("var a = 1;\na = a / 0;\na\n").0, ["1"]);
}

#[test]
fn stack_overflow_is_reported_without_a_place() {
    let (_, err) = values("fn f(): int { return f() + 1; }\nf()\n");

    assert_eq!(err, "runtime error: stack overflow: the calls nest too deeply\n");
}

#[test]
fn comment_only_entry_does_nothing() {
    let (values, err) = values("// just a comment\n/* and another */\n1\n");

    assert_eq!(values, ["1"]);
    assert_eq!(err, "");
}

#[test]
fn end_of_input_ends_the_prompt() {
    let (code, out, _) = session("");

    assert_eq!(code, OK);
    assert!(out.starts_with(BANNER), "{out}");
}

#[test]
fn open_entries_are_unfinished() {
    for text in ["{", "f(", "a[", "fn f() {\n  if x {", "`multi", "/* comment", "{ /* } */"] {
        assert!(unfinished(text), "{text:?}");
    }
}

#[test]
fn closed_entries_are_finished() {
    for text in ["", "1", "{}", "f()", "}", "`done`", "/* done */", "\"{\"", "// {"] {
        assert!(!unfinished(text), "{text:?}");
    }
}
