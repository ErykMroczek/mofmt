use mofmt::{format, pretty_print};
use moparse::{lex, parse, SyntaxKind};
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = read_file(file_path);
    let tokens = lex(&contents);
    let events = parse(&tokens, SyntaxKind::StoredDefinition);
    let markers = format(&tokens, &events);
    let output = pretty_print(&tokens, &markers);
    write_file(file_path, output);
}

/// Return all Modelica files from the given directory
fn get_files_from_dir(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let paths = fs::read_dir(dir)
        .expect(format!("{}: error reading from a directory", dir.display()).as_str());
    for item in paths {
        match item {
            Ok(entry) => {
                let path = entry.path();
                if path.is_file() && path.extension().is_some() {
                    if path.extension().unwrap() == "mo" {
                        files.push(path);
                    }
                } else if path.is_dir() {
                    files.append(&mut get_files_from_dir(path.as_path()));
                }
            }
            Err(_) => (),
        }
    }

    files
}

/// Return contents of the Modelica file
fn read_file(from: &String) -> String {
    let path = Path::new(&from);
    let suffix = path
        .extension()
        .expect(format!("{}: is not a Modelica file", from).as_str());
    if suffix != "mo" {
        panic!("{}: is not a Modelica file", from);
    }

    fs::read_to_string(path).expect(format!("{}: error reading a file", from).as_str())
}

/// Write formatted code to a file
fn write_file(to: &String, code: String) {
    fs::write(to, code).expect(format!("{}: error writing a file", to).as_str());
}
