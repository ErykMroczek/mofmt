use mofmt;
use moparse;
use std::fs;

// Helper functions
fn format_file(path: &str) -> String {
    let input = fs::read_to_string(path).expect("error");
    let parsed = moparse::parse(&input, moparse::SyntaxKind::StoredDefinition);
    mofmt::pretty_print(parsed.tokens, parsed.comments, parsed.events)
}

#[test]
fn test_formatting() {
    let formatted = format_file("tests/samples/code-input.mo");
    let expected = fs::read_to_string("tests/samples/code-output.mo").expect("error");
    assert_eq!(expected, formatted);
}
