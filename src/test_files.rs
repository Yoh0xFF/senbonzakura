use std::fs;

///
/// A single test case split out of a test file, with its expected output still as raw text
///
/// Test files contain one or more cases in the following layout:
///
/// ```text
/// === Description of the test
/// === that may continue on further `===` lines
/// source code;
/// ---
/// expected output
/// ```
///
/// A case starts with one or more lines prefixed with `===`, which are joined into its
/// description. The source runs until a line of three or more `-`, and the expected output
/// runs until the next header. Anything before the first header is ignored.
///
#[derive(Debug)]
pub(crate) struct RawTestCase {
    pub line: usize,
    pub expected_line: usize,
    pub description: String,
    pub source: String,
    pub expected: String,
}

const HEADER_PREFIX: &str = "===";

fn is_header(line: &str) -> bool {
    line.starts_with(HEADER_PREFIX)
}

fn is_separator(line: &str) -> bool {
    let line = line.trim_end();
    line.len() >= 3 && line.chars().all(|c| c == '-')
}

pub(crate) fn split_test_cases(content: &str) -> Result<Vec<RawTestCase>, String> {
    let mut lines = content.lines().enumerate().peekable();
    let mut test_cases = Vec::new();

    // Skip anything before the first header
    while lines.next_if(|(_, line)| !is_header(line)).is_some() {}

    while let Some(&(header_line, _)) = lines.peek() {
        let mut description = Vec::new();
        while let Some((_, line)) = lines.next_if(|(_, line)| is_header(line)) {
            let part = line[HEADER_PREFIX.len()..].trim();
            if !part.is_empty() {
                description.push(part);
            }
        }

        let mut source = String::new();
        let expected_line = loop {
            match lines.next() {
                Some((index, line)) if is_separator(line) => break index + 2,
                Some((_, line)) => {
                    source.push_str(line);
                    source.push('\n');
                }
                None => {
                    return Err(format!(
                        "Missing '---' separator in test case at line {}",
                        header_line + 1
                    ))
                }
            }
        };

        let mut expected = String::new();
        while let Some((_, line)) = lines.next_if(|(_, line)| !is_header(line)) {
            expected.push_str(line);
            expected.push('\n');
        }

        test_cases.push(RawTestCase {
            line: header_line + 1,
            expected_line,
            description: description.join(" "),
            source,
            expected,
        });
    }

    Ok(test_cases)
}

///
/// Read a test file and parse every case in it with `parse_case`, panicking on any error
///
pub(crate) fn load_test_cases<T>(
    path: &str,
    parse_case: impl Fn(RawTestCase) -> Result<T, String>,
) -> Vec<T> {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Failed to read the test file: {}", error);
            eprintln!("Path: {}", path);
            panic!("Could not read test file: {}", error);
        }
    };

    let test_cases = match split_test_cases(&content).and_then(|cases| {
        cases
            .into_iter()
            .map(&parse_case)
            .collect::<Result<Vec<_>, _>>()
    }) {
        Ok(test_cases) => test_cases,
        Err(error) => {
            eprintln!("Path: {}", path);
            panic!("Could not parse test file: {}", error);
        }
    };

    // Verify that we found some test cases
    if test_cases.is_empty() {
        eprintln!("No test cases found in file: {}", path);
        panic!("File contained no valid test cases");
    }

    test_cases
}

#[test]
fn test_split_test_cases() {
    let content = "\
leading notes are ignored
=== First
1;
---
expected one

=== Second case
===   spanning two lines
2;
-----
expected two
";
    let test_cases = split_test_cases(content).unwrap();
    assert_eq!(test_cases.len(), 2);
    assert_eq!(test_cases[0].line, 2);
    assert_eq!(test_cases[0].expected_line, 5);
    assert_eq!(test_cases[0].description, "First");
    assert_eq!(test_cases[0].source, "1;\n");
    assert_eq!(test_cases[0].expected, "expected one\n\n");
    assert_eq!(test_cases[1].description, "Second case spanning two lines");
    assert_eq!(test_cases[1].expected, "expected two\n");
}

#[test]
fn test_split_test_cases_missing_separator() {
    assert!(split_test_cases("=== A\n1;\n").is_err());
}
