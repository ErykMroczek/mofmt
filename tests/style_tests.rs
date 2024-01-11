use mofmt;
use moparse;
use std::fs;

// Helper functions
fn format_file(path: &str) -> String {
    let input = fs::read_to_string(path).expect("error");
    let tokens = moparse::lex(&input);
    let events = moparse::parse(&tokens, moparse::SyntaxKind::StoredDefinition);
    let markers = mofmt::format(&tokens, &events);
    mofmt::pretty_print(&tokens, markers)
}

#[test]
fn models_formatting() {
    let formatted = format_file("tests/samples/models-input.mo");
    let expected = fs::read_to_string("tests/samples/models-output.mo").expect("error");
    assert_eq!(expected, formatted);
}
#[test]
fn functions_formatting() {
    let formatted = format_file("tests/samples/functions-input.mo");
    let expected = fs::read_to_string("tests/samples/functions-output.mo").expect("error");
    assert_eq!(expected, formatted);
}
