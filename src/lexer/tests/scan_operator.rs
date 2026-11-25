use super::{Diagnostic, Kind, lex};

fn operators() -> impl Iterator<Item = Kind> {
    Kind::ALL.iter().copied().filter(|kind| kind.is_operator())
}

#[test]
fn every_operator_is_one_token() {
    for operator in operators() {
        let spelling = operator.spelling().unwrap();
        let lexed = lex(spelling);

        let token = lexed.single();
        assert_eq!(token.kind, operator);
        assert_eq!(lexed.text(token), spelling);
        assert_eq!(token.flags, 0);
        assert_eq!(lexed.messages(), Vec::<&str>::new());
    }
}

#[test]
fn spellings_are_ascii() {
    // The scanner skips over a match with Cursor::bump_n, which is only correct for ASCII.
    for operator in operators() {
        assert!(operator.spelling().unwrap().is_ascii(), "{operator:?}");
    }
}

#[test]
fn longest_match() {
    let cases: [(&str, &[Kind]); 16] = [
        ("<=", &[Kind::LessEq]),
        ("< =", &[Kind::Less, Kind::Eq]),
        ("===", &[Kind::EqEq, Kind::Eq]),
        ("!==", &[Kind::BangEq, Kind::Eq]),
        ("!!", &[Kind::Bang, Kind::Bang]),
        ("...", &[Kind::DotDot, Kind::Dot]),
        ("-->", &[Kind::MinusMinus, Kind::Greater]),
        ("- ->", &[Kind::Minus, Kind::Arrow]),
        ("+++", &[Kind::PlusPlus, Kind::Plus]),
        ("---", &[Kind::MinusMinus, Kind::Minus]),
        ("++=", &[Kind::PlusPlus, Kind::Eq]),
        ("->=", &[Kind::Arrow, Kind::Eq]),
        ("+=+", &[Kind::PlusEq, Kind::Plus]),
        ("<<", &[Kind::Less, Kind::Less]),
        (">>=", &[Kind::Greater, Kind::GreaterEq]),
        ("||||", &[Kind::OrOr, Kind::OrOr]),
    ];
    for (input, kinds) in cases {
        let lexed = lex(input);

        assert_eq!(lexed.kinds(), kinds, "{input:?}");
        assert_eq!(lexed.messages(), Vec::<&str>::new());
    }
}

#[test]
fn characters_that_start_no_operator_are_errors() {
    for input in ["&", "|", "#", "@", "?", "\\"] {
        let lexed = lex(input);

        assert_eq!(lexed.single().kind, Kind::Error, "{input:?}");
        assert_eq!(lexed.diagnostics, [Diagnostic::new(0, 1, "unexpected character")]);
    }
}

#[test]
fn partial_match_falls_back_to_an_error() {
    let lexed = lex("&&&");

    assert_eq!(lexed.kinds(), [Kind::AndAnd, Kind::Error]);
    assert_eq!(lexed.diagnostic_texts(), ["&"]);
}

#[test]
fn operators_between_other_tokens() {
    let cases: [(&str, &[Kind], &[&str]); 6] = [
        ("a+=1", &[Kind::Identifier, Kind::PlusEq, Kind::IntLiteral], &["a", "+=", "1"]),
        (
            "f(x,y);",
            &[
                Kind::Identifier,
                Kind::LeftParen,
                Kind::Identifier,
                Kind::Comma,
                Kind::Identifier,
                Kind::RightParen,
                Kind::Semicolon,
            ],
            &["f", "(", "x", ",", "y", ")", ";"],
        ),
        ("0..10", &[Kind::IntLiteral, Kind::DotDot, Kind::IntLiteral], &["0", "..", "10"]),
        ("x.len", &[Kind::Identifier, Kind::Dot, Kind::Identifier], &["x", ".", "len"]),
        ("-1", &[Kind::Minus, Kind::IntLiteral], &["-", "1"]),
        (
            "a[i]!=\"s\"",
            &[
                Kind::Identifier,
                Kind::LeftBracket,
                Kind::Identifier,
                Kind::RightBracket,
                Kind::BangEq,
                Kind::StringLiteral,
            ],
            &["a", "[", "i", "]", "!=", "\"s\""],
        ),
    ];
    for (input, kinds, texts) in cases {
        let lexed = lex(input);

        assert_eq!(lexed.kinds(), kinds, "{input:?}");
        assert_eq!(lexed.texts(), texts);
        assert_eq!(lexed.messages(), Vec::<&str>::new());
    }
}

#[test]
fn small_program() {
    let lexed = lex("fn add(a: Int, b: Int) -> Int {\n  return a + b;\n}\n");

    assert_eq!(
        lexed.kinds(),
        [
            Kind::Fn,
            Kind::Identifier,
            Kind::LeftParen,
            Kind::Identifier,
            Kind::Colon,
            Kind::Identifier,
            Kind::Comma,
            Kind::Identifier,
            Kind::Colon,
            Kind::Identifier,
            Kind::RightParen,
            Kind::Arrow,
            Kind::Identifier,
            Kind::LeftBrace,
            Kind::Return,
            Kind::Identifier,
            Kind::Plus,
            Kind::Identifier,
            Kind::Semicolon,
            Kind::RightBrace,
        ]
    );
    assert_eq!(lexed.messages(), Vec::<&str>::new());
}
