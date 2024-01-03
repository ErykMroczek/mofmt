use moparse::{parse, lex, SyntaxKind};
use mofmt::{pretty_print, format};
use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path)
        .expect("error");
    let tokens = lex(&contents);
    let events = parse(&tokens, SyntaxKind::StoredDefinition);
    let markers = format(&tokens, &events);
    let output = pretty_print(&tokens, &markers);
    println!("{}", output);
}
