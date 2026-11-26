use super::{Parser, parse_all, parse_stmt, parse_stmt_clean};
use senbonzakura_ast::{Stmt, sexpr};
use senbonzakura_lexer::{Diagnostic, Lexer, SourceFile};

fn assert_trees(cases: &[(&str, &str)]) {
    for &(input, tree) in cases {
        assert_eq!(parse_stmt_clean(input).tree(), tree, "{input:?}");
    }
}

/// Each input is one statement with one problem, reported with the message given.
fn assert_reported_once(cases: &[(&str, &str, &str)]) {
    for &(input, tree, message) in cases {
        let parsed = parse_stmt(input);

        assert_eq!(parsed.tree(), tree, "{input:?}");
        assert_eq!(parsed.parser_messages(), [message], "{input:?}");
    }
}

/// Each input is one statement missing only a zero-width something at `offset`.
fn assert_missing_at(cases: &[(&str, &str, usize, &str)]) {
    for &(input, tree, offset, message) in cases {
        let parsed = parse_stmt(input);

        assert_eq!(parsed.tree(), tree, "{input:?}");
        assert_eq!(
            parsed.parser_diagnostics,
            [Diagnostic::new(offset, offset, message)],
            "{input:?}"
        );
    }
}

#[test]
fn expression_statements() {
    assert_trees(&[
        ("1;", "1;"),
        ("x = 1 + 2;", "(= x (+ 1 2));"),
        ("a..b;", "(.. a b);"),
        ("(x);", "(group x);"),
    ]);
}

#[test]
fn blocks() {
    assert_trees(&[
        ("{}", "{}"),
        ("{ x; }", "{x;}"),
        ("{ x = 1; y += 2; }", "{(= x 1); (+= y 2);}"),
        ("{ a; { b; } c; }", "{a; {b;} c;}"),
        ("{{}}", "{{}}"),
    ]);
}

#[test]
fn statements_cover_their_source() {
    let parsed = parse_stmt_clean("{ x = 1; { y; } }");
    let Stmt::Block(outer) = &parsed.stmt else { panic!() };

    assert_eq!(parsed.text(outer), "{ x = 1; { y; } }");
    assert_eq!(parsed.text(&outer.statements[0]), "x = 1;");
    assert_eq!(parsed.text(&outer.statements[1]), "{ y; }");
}

#[test]
fn missing_semicolon_is_reported_after_the_expression() {
    let message = "expected ';' after expression";
    assert_missing_at(&[
        ("x", "x;", 1, message),
        ("x = 1 + 2", "(= x (+ 1 2));", 9, message),
        ("{ x }", "{x;}", 3, message),
        ("{ a b; }", "{a; b;}", 3, message),
    ]);
}

#[test]
fn missing_semicolon_is_not_reported_after_a_broken_expression() {
    assert_reported_once(&[
        ("x = ;", "(= x <error>);", "expected expression, found ';'"),
        ("1 +", "(+ 1 <error>);", "expected expression, found end of input"),
        (";", "<error>;", "expected expression, found ';'"),
        ("{ 1 + }", "{(+ 1 <error>);}", "expected expression, found '}'"),
    ]);
}

#[test]
fn unclosed_block_is_reported() {
    let parsed = parse_stmt("{ x;");

    assert_eq!(parsed.tree(), "{x;}");
    assert_eq!(parsed.parser_messages(), ["expected '}', found end of input"]);
    assert_eq!(parsed.text(&parsed.stmt), "{ x;");
}

#[test]
fn unclosed_empty_block_ends_at_its_brace() {
    let parsed = parse_stmt("{");

    assert_eq!(parsed.tree(), "{}");
    assert_eq!(parsed.text(&parsed.stmt), "{");
}

#[test]
fn stray_closer_inside_a_block_is_reported_and_skipped() {
    let parsed = parse_stmt("{ ) x; }");

    assert_eq!(parsed.tree(), "{<error>; x;}");
    assert_eq!(parsed.parser_messages(), ["expected expression, found ')'"]);
}

#[test]
fn stray_close_brace_is_reported_and_skipped() {
    let parsed = parse_stmt("}");

    assert_eq!(parsed.parser_messages(), ["expected expression, found '}'"]);
}

#[test]
fn parses_one_statement_per_call() {
    let source = SourceFile::new("test", "a; { b; }\nc = 1;");
    let mut parser = Parser::new(Lexer::new(&source));

    let mut trees = Vec::new();
    while !parser.at_end() {
        trees.push(sexpr::stmt(&parser.parse_statement()));
    }

    assert_eq!(trees, ["a;", "{b;}", "(= c 1);"]);
    assert!(parser.diagnostics().is_empty());
}

#[test]
fn var_statements() {
    assert_trees(&[
        ("var x = 1;", "(var x 1)"),
        ("var x;", "(var x)"),
        ("var mut = 1;", "(var mut 1)"),
        ("var while = 0;", "(var while 0)"),
        ("var for;", "(var for)"),
        ("var let = 1;", "(var let 1)"),
        ("var x = a + f(b);", "(var x (+ a (call f b)))"),
        ("var r = 0..n;", "(var r (.. 0 n))"),
        ("{ var x = 1; x += 2; }", "{(var x 1) (+= x 2);}"),
    ]);
}

#[test]
fn var_covers_its_source() {
    let parsed = parse_stmt_clean("var x = 1 + 2;");
    let Stmt::Var(var) = &parsed.stmt else { panic!() };

    assert_eq!(var.declarators[0].name, "x");
    assert_eq!(parsed.text(var), "var x = 1 + 2;");
}

#[test]
fn typed_and_multiple_var_statements() {
    assert_trees(&[
        ("var i: int;", "(var i: int)"),
        ("var i = 0;", "(var i 0)"),
        ("var i: int = 0;", "(var i: int 0)"),
        ("var i: int = 0, j: int = 0;", "(var i: int 0, j: int 0)"),
        ("var a, b = 1, c: bool;", "(var a, b 1, c: bool)"),
        ("var f: double = 1.5 * x;", "(var f: double (* 1.5 x))"),
        ("{ var a: int, b: uint; a; }", "{(var a: int, b: uint) a;}"),
    ]);
}

#[test]
fn declarators_cover_their_source() {
    let parsed = parse_stmt_clean("var i: int = 0, j = 1;");
    let Stmt::Var(var) = &parsed.stmt else { panic!() };
    let (i, j) = (&var.declarators[0], &var.declarators[1]);

    assert_eq!(parsed.text(var), "var i: int = 0, j = 1;");
    assert_eq!(parsed.text(i), "i: int = 0");
    assert_eq!(parsed.text(i.ty.as_ref().unwrap()), "int");
    assert_eq!(parsed.text(j), "j = 1");
    assert_eq!(j.ty, None);
}

#[test]
fn malformed_typed_var_is_reported_once() {
    assert_reported_once(&[
        ("var i: = 0;", "(var i: <error> 0)", "expected type, found '='"),
        ("var i:;", "(var i: <error>)", "expected type, found ';'"),
        ("var i: foo = 0;", "(var i: <error> 0)", "expected type, found 'foo'"),
        ("var i: 1 = 0;", "(var i: <error> 0)", "expected type, found '1'"),
        ("var i: int, j:;", "(var i: int, j: <error>)", "expected type, found ';'"),
        ("var : int = 0;", "(var <error>: int 0)", "expected variable name, found ':'"),
        ("var i = 0, ;", "(var i 0, <error>)", "expected variable name, found ';'"),
        ("var i = 0,, j;", "(var i 0, <error>, j)", "expected variable name, found ','"),
        ("var i = , j;", "(var i <error>, j)", "expected expression, found ','"),
    ]);
}

#[test]
fn missing_semicolon_after_a_type_is_reported_after_it() {
    assert_missing_at(&[("var x: int", "(var x: int)", 10, "expected ';' after var statement")]);
}

#[test]
fn malformed_var_is_reported_once() {
    assert_reported_once(&[
        ("var = 1;", "(var <error> 1)", "expected variable name, found '='"),
        ("var;", "(var <error>)", "expected variable name, found ';'"),
        ("var fn = 1;", "(var <error> 1)", "expected variable name, found 'fn'"),
        ("var 1 = 2;", "(var <error> 2)", "expected variable name, found '1'"),
        ("var x = ;", "(var x <error>)", "expected expression, found ';'"),
    ]);
}

#[test]
fn missing_semicolon_after_var_is_reported() {
    let message = "expected ';' after var statement";
    assert_missing_at(&[
        ("var x = 1", "(var x 1)", 9, message),
        ("var x", "(var x)", 5, message),
        ("{ var x 1; }", "{(var x) 1;}", 7, message),
    ]);
}

#[test]
fn var_is_not_an_expression() {
    let parsed = parse_stmt("{ x = var y; }");

    assert_eq!(parsed.parser_messages(), ["expected expression, found 'var'"]);
}

#[test]
fn if_statements() {
    assert_trees(&[
        ("if a { x; }", "(if a {x;})"),
        ("if a {}", "(if a {})"),
        ("if a == b || c == d { x = 1; }", "(if (|| (== a b) (== c d)) {(= x 1);})"),
        ("if f(x) && !y.z { }", "(if (&& (call f x) (! (. y z))) {})"),
        ("if (a) { b; }", "(if (group a) {b;})"),
        ("if a { if b { c; } }", "(if a {(if b {c;})})"),
        ("{ if a { b; } c; }", "{(if a {b;}) c;}"),
    ]);
}

#[test]
fn if_covers_its_source() {
    let parsed = parse_stmt_clean("if a < b { x; }");
    let Stmt::If(s) = &parsed.stmt else { panic!() };

    assert_eq!(parsed.text(&parsed.stmt), "if a < b { x; }");
    assert_eq!(parsed.text(&s.condition), "a < b");
    assert_eq!(parsed.text(&s.then), "{ x; }");
}

#[test]
fn if_needs_no_semicolon() {
    let all = parse_all("if a { b; } c;");

    assert_eq!(all.trees, ["(if a {b;})", "c;"]);
    assert_eq!(all.messages, Vec::<String>::new());
}

#[test]
fn malformed_if_is_reported_once() {
    assert_reported_once(&[
        ("if { x; }", "(if <error> {x;})", "expected expression, found '{'"),
        ("if a + { }", "(if (+ a <error>) {})", "expected expression, found '{'"),
        ("if a +", "(if (+ a <error>) {})", "expected expression, found end of input"),
        ("if a", "(if a {})", "expected '{' after if condition, found end of input"),
        ("if a { x;", "(if a {x;})", "expected '}', found end of input"),
    ]);
}

#[test]
fn missing_body_leaves_the_rest_for_the_next_statement() {
    let all = parse_all("if a b; c;");

    assert_eq!(all.trees, ["(if a {})", "b;", "c;"]);
    assert_eq!(all.messages, ["expected '{' after if condition, found 'b'"]);
}

#[test]
fn missing_body_is_an_empty_block_at_the_end_of_the_condition() {
    let parsed = parse_stmt("if a ");
    let Stmt::If(s) = &parsed.stmt else { panic!() };

    assert_eq!(parsed.text(&parsed.stmt), "if a");
    assert_eq!((s.then.start, s.then.end), (4, 4));
}

#[test]
fn if_else_statements() {
    assert_trees(&[
        ("if a { x; } else { y; }", "(if a {x;} else {y;})"),
        ("if a {} else {}", "(if a {} else {})"),
        ("if a {} else if b { x; }", "(if a {} else (if b {x;}))"),
        ("if a {} else if b {} else {}", "(if a {} else (if b {} else {}))"),
        ("if a { if b {} else {} }", "(if a {(if b {} else {})})"),
        ("{ if a {} else {} b; }", "{(if a {} else {}) b;}"),
    ]);
}

#[test]
fn if_else_covers_its_source() {
    let parsed = parse_stmt_clean("if a { x; } else if b { y; } else { z; }");
    let Stmt::If(s) = &parsed.stmt else { panic!() };
    let Some(Stmt::If(else_if)) = s.otherwise.as_deref() else { panic!() };

    assert_eq!(parsed.text(&parsed.stmt), "if a { x; } else if b { y; } else { z; }");
    assert_eq!(parsed.text(s.otherwise.as_deref().unwrap()), "if b { y; } else { z; }");
    assert_eq!(parsed.text(else_if.otherwise.as_deref().unwrap()), "{ z; }");
}

#[test]
fn malformed_else_is_reported_once() {
    assert_reported_once(&[
        (
            "if a {} else",
            "(if a {} else {})",
            "expected '{' or 'if' after 'else', found end of input",
        ),
        ("if a else {}", "(if a {} else {})", "expected '{' after if condition, found 'else'"),
        ("if a {} else if {}", "(if a {} else (if <error> {}))", "expected expression, found '{'"),
        ("if a {} else { x;", "(if a {} else {x;})", "expected '}', found end of input"),
    ]);
}

#[test]
fn missing_else_body_leaves_the_rest_for_the_next_statement() {
    let all = parse_all("if a {} else b; c;");

    assert_eq!(all.trees, ["(if a {} else {})", "b;", "c;"]);
    assert_eq!(all.messages, ["expected '{' or 'if' after 'else', found 'b'"]);
}

#[test]
fn missing_else_body_is_an_empty_block_after_the_else() {
    let parsed = parse_stmt("if a {} else ");
    let Stmt::If(s) = &parsed.stmt else { panic!() };
    let otherwise = s.otherwise.as_deref().unwrap();

    assert_eq!(parsed.text(&parsed.stmt), "if a {} else");
    assert_eq!((otherwise.start(), otherwise.end()), (12, 12));
}

#[test]
fn else_without_if_is_reported() {
    let all = parse_all("else { x; }");

    assert_eq!(all.messages, ["expected expression, found 'else'"]);
}

#[test]
fn loop_statements() {
    assert_trees(&[
        ("loop {}", "(loop {})"),
        ("loop { x; }", "(loop {x;})"),
        ("loop a == b { x; }", "(loop (== a b) {x;})"),
        ("loop !done && n < 10 {}", "(loop (&& (! done) (< n 10)) {})"),
        ("loop var i in 0..10 { x; }", "(loop var i in (.. 0 10) {x;})"),
        ("loop var c in s.chars() {}", "(loop var c in (call (. s chars)) {})"),
        ("loop var i = 0; i < 10; i++ { x; }", "(loop (var i 0) (< i 10) (++ i) {x;})"),
        ("loop var i = 9; i >= 0; i-- {}", "(loop (var i 9) (>= i 0) (-- i) {})"),
        ("loop var i = 0; i < n; i += 2 {}", "(loop (var i 0) (< i n) (+= i 2) {})"),
        ("{ loop a { b; } c; }", "{(loop a {b;}) c;}"),
        ("loop a { loop var x in xs { if x {} } }", "(loop a {(loop var x in xs {(if x {})})})"),
    ]);
}

#[test]
fn loop_covers_its_source() {
    let parsed = parse_stmt_clean("loop a < b { x; }");
    let Stmt::Loop(s) = &parsed.stmt else { panic!() };

    assert_eq!(parsed.text(&parsed.stmt), "loop a < b { x; }");
    assert_eq!(parsed.text(s.condition.as_ref().unwrap()), "a < b");
    assert_eq!(parsed.text(&s.body), "{ x; }");
}

#[test]
fn loop_in_covers_its_source() {
    let parsed = parse_stmt_clean("loop var i in 0..n { x; }");
    let Stmt::LoopIn(s) = &parsed.stmt else { panic!() };

    assert_eq!(s.name, "i");
    assert_eq!(parsed.text(&parsed.stmt), "loop var i in 0..n { x; }");
    assert_eq!(parsed.text(&s.iterable), "0..n");
}

#[test]
fn loop_step_covers_its_source() {
    let parsed = parse_stmt_clean("loop var i = 0; i < 10; i++ { x; }");
    let Stmt::LoopStep(s) = &parsed.stmt else { panic!() };

    assert_eq!(parsed.text(&parsed.stmt), "loop var i = 0; i < 10; i++ { x; }");
    assert_eq!(parsed.text(&s.init), "var i = 0;");
    assert_eq!(parsed.text(&s.condition), "i < 10");
    assert_eq!(parsed.text(&s.step), "i++");
    assert_eq!(parsed.text(&s.body), "{ x; }");
}

#[test]
fn loop_variables_take_types() {
    assert_trees(&[
        ("loop var c: char in s {}", "(loop var c: char in s {})"),
        (
            "loop var i: int = 0, j = 9; i < j; i++ {}",
            "(loop (var i: int 0, j 9) (< i j) (++ i) {})",
        ),
        ("loop var i = 0, j: int; i < 9; i++ {}", "(loop (var i 0, j: int) (< i 9) (++ i) {})"),
    ]);
}

#[test]
fn loop_variable_with_a_missing_type_is_reported_once() {
    assert_reported_once(&[(
        "loop var i: in xs {}",
        "(loop var i: <error> in xs {})",
        "expected type, found 'in'",
    )]);
}

#[test]
fn loop_needs_no_semicolon() {
    let all = parse_all("loop a { b; } loop {} c;");

    assert_eq!(all.trees, ["(loop a {b;})", "(loop {})", "c;"]);
    assert_eq!(all.messages, Vec::<String>::new());
}

#[test]
fn malformed_loop_is_reported_once() {
    assert_reported_once(&[
        ("loop a", "(loop a {})", "expected '{' after loop condition, found end of input"),
        ("loop a +", "(loop (+ a <error>) {})", "expected expression, found end of input"),
        ("loop { x;", "(loop {x;})", "expected '}', found end of input"),
        ("loop var in xs {}", "(loop var <error> in xs {})", "expected variable name, found 'in'"),
        (
            "loop var i {}",
            "(loop var i in <error> {})",
            "expected 'in' or '=' after loop variable, found '{'",
        ),
        (
            "loop var i xs {}",
            "(loop var i in xs {})",
            "expected 'in' or '=' after loop variable, found 'xs'",
        ),
        ("loop var i in {}", "(loop var i in <error> {})", "expected expression, found '{'"),
        (
            "loop var i in xs",
            "(loop var i in xs {})",
            "expected '{' after loop iterable, found end of input",
        ),
        (
            "loop var i = 0 i < 9; i++ {}",
            "(loop (var i 0) (< i 9) (++ i) {})",
            "expected ';' after var statement",
        ),
        (
            "loop var i = 0; i < 9 i++ {}",
            "(loop (var i 0) (< i 9) (++ i) {})",
            "expected ';' after loop condition",
        ),
        (
            "loop var i = 0; i < 9; {}",
            "(loop (var i 0) (< i 9) <error> {})",
            "expected expression, found '{'",
        ),
        (
            "loop var i = 0; i < 9; i++",
            "(loop (var i 0) (< i 9) (++ i) {})",
            "expected '{' after loop step, found end of input",
        ),
    ]);
}

#[test]
fn missing_loop_body_leaves_the_rest_for_the_next_statement() {
    let all = parse_all("loop a b; c;");

    assert_eq!(all.trees, ["(loop a {})", "b;", "c;"]);
    assert_eq!(all.messages, ["expected '{' after loop condition, found 'b'"]);
}

#[test]
fn plain_var_skips_an_in_where_the_name_should_be() {
    assert_reported_once(&[(
        "var in = 1;",
        "(var <error> 1)",
        "expected variable name, found 'in'",
    )]);
}

#[test]
fn continue_and_break_statements() {
    assert_trees(&[
        ("continue;", "(continue)"),
        ("continue if a == b;", "(continue if (== a b))"),
        ("continue if i >= 10;", "(continue if (>= i 10))"),
        ("continue if (a > b && c < d);", "(continue if (group (&& (> a b) (< c d))))"),
        ("continue if !f(x);", "(continue if (! (call f x)))"),
        ("break;", "(break)"),
        ("break if a == b;", "(break if (== a b))"),
        ("break if i >= 10;", "(break if (>= i 10))"),
        ("break if (a > b && c < d);", "(break if (group (&& (> a b) (< c d))))"),
        ("loop { continue if a; break if b; x; }", "(loop {(continue if a) (break if b) x;})"),
    ]);
}

#[test]
fn continue_and_break_cover_their_source() {
    for input in ["continue if a < b;", "break if a < b;"] {
        let parsed = parse_stmt_clean(input);
        let (Stmt::Continue(s) | Stmt::Break(s)) = &parsed.stmt else { panic!() };

        assert_eq!(parsed.text(&parsed.stmt), input);
        assert_eq!(parsed.text(s.condition.as_ref().unwrap()), "a < b");
    }
}

#[test]
fn missing_semicolon_after_continue_or_break_is_reported() {
    assert_missing_at(&[
        ("continue", "(continue)", 8, "expected ';' or 'if' after 'continue'"),
        ("continue if a", "(continue if a)", 13, "expected ';' after continue condition"),
        ("{ continue }", "{(continue)}", 10, "expected ';' or 'if' after 'continue'"),
        ("break", "(break)", 5, "expected ';' or 'if' after 'break'"),
        ("break if a", "(break if a)", 10, "expected ';' after break condition"),
        ("{ break }", "{(break)}", 7, "expected ';' or 'if' after 'break'"),
    ]);
}

#[test]
fn missing_condition_is_reported_once() {
    for keyword in ["continue", "break", "return"] {
        let parsed = parse_stmt(&format!("{keyword} if ;"));

        assert_eq!(parsed.tree(), format!("({keyword} if <error>)"));
        assert_eq!(parsed.parser_messages(), ["expected expression, found ';'"]);
    }
}

#[test]
fn continue_or_break_leaves_what_follows_for_the_next_statement() {
    for keyword in ["continue", "break"] {
        let all = parse_all(&format!("{keyword} x; y;"));

        assert_eq!(all.trees, [format!("({keyword})"), "x;".to_string(), "y;".to_string()]);
        assert_eq!(all.messages, [format!("expected ';' or 'if' after '{keyword}'")]);
    }
}

#[test]
fn continue_break_and_return_are_not_expressions() {
    for keyword in ["continue", "break", "return"] {
        let parsed = parse_stmt(&format!("{{ x = {keyword}; }}"));

        assert_eq!(parsed.parser_messages(), [format!("expected expression, found '{keyword}'")]);
    }
}

#[test]
fn return_statements() {
    assert_trees(&[
        ("return;", "(return)"),
        ("return if a == b;", "(return if (== a b))"),
        ("return if i >= 10;", "(return if (>= i 10))"),
        ("return if (a > b && c < d);", "(return if (group (&& (> a b) (< c d))))"),
        ("return x;", "(return x)"),
        ("return x if a == b;", "(return x if (== a b))"),
        ("return x if i >= 10;", "(return x if (>= i 10))"),
        ("return x if (a > b && c < d);", "(return x if (group (&& (> a b) (< c d))))"),
        ("return a + f(b);", "(return (+ a (call f b)))"),
        ("return (x) if y;", "(return (group x) if y)"),
        ("return a == b if c;", "(return (== a b) if c)"),
        ("return a, b;", "(return a, b)"),
        ("return a, b if c;", "(return a, b if c)"),
        ("return x + 1, f(y), z;", "(return (+ x 1), (call f y), z)"),
        ("{ return x if a; y; }", "{(return x if a) y;}"),
    ]);
}

#[test]
fn return_covers_its_source() {
    let parsed = parse_stmt_clean("return x + 1 if a < b;");
    let Stmt::Return(s) = &parsed.stmt else { panic!() };

    assert_eq!(parsed.text(&parsed.stmt), "return x + 1 if a < b;");
    assert_eq!(parsed.text(&s.values[0]), "x + 1");
    assert_eq!(parsed.text(s.condition.as_ref().unwrap()), "a < b");
}

#[test]
fn each_return_value_covers_its_source() {
    let parsed = parse_stmt_clean("return a + 1, f(b) if c;");
    let Stmt::Return(s) = &parsed.stmt else { panic!() };

    assert_eq!(s.values.len(), 2);
    assert_eq!(parsed.text(&s.values[0]), "a + 1");
    assert_eq!(parsed.text(&s.values[1]), "f(b)");
    assert_eq!(parsed.text(&parsed.stmt), "return a + 1, f(b) if c;");
}

#[test]
fn missing_semicolon_after_return_is_reported() {
    assert_missing_at(&[
        ("return", "(return)", 6, "expected ';' or 'if' after 'return'"),
        ("{ return }", "{(return)}", 8, "expected ';' or 'if' after 'return'"),
        ("return x", "(return x)", 8, "expected ';' or 'if' after return value"),
        ("return if a", "(return if a)", 11, "expected ';' after return condition"),
        ("return x if a", "(return x if a)", 13, "expected ';' after return condition"),
    ]);
}

#[test]
fn malformed_return_is_reported_once() {
    assert_reported_once(&[
        ("return x if ;", "(return x if <error>)", "expected expression, found ';'"),
        ("return x +;", "(return (+ x <error>))", "expected expression, found ';'"),
        ("return x + if a;", "(return (+ x <error>) if a)", "expected expression, found 'if'"),
        ("return a, ;", "(return a, <error>)", "expected expression, found ';'"),
        ("return a, if c;", "(return a, <error> if c)", "expected expression, found 'if'"),
        ("return , a;", "(return <error>, a)", "expected expression, found ','"),
    ]);
}

#[test]
fn return_leaves_what_follows_its_value_for_the_next_statement() {
    let all = parse_all("return x y; z;");

    assert_eq!(all.trees, ["(return x)", "y;", "z;"]);
    assert_eq!(all.messages, ["expected ';' or 'if' after return value"]);
}
