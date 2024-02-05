use mofmt::pretty_print;
use moparse::{parse, SyntaxKind};
use std::path::{Path, PathBuf};
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    format_files(&args[1..]);
}

/// Format files specified in the argument list
fn format_files(args: &[String]) {
    let mut files = Vec::new();
    args.iter()
        .map(PathBuf::from)
        .map(|p| {
            if p.is_dir() {
                get_files_from_dir(p)
            } else {
                vec![p]
            }
        })
        .for_each(|mut v| files.append(&mut v));
    files.iter().for_each(|p| {
        let contents = read_file(p);
        let parsed = parse(&contents, SyntaxKind::StoredDefinition);
        if !parsed.errors.is_empty() {
            let messages: Vec<String> = parsed.errors
                .iter()
                .map(|e| format!("{}:{}", p.display(), e))
                .collect();
            panic!("Syntax errors detected:\n{}", messages.join("\n"));
        }
        let output = pretty_print(parsed.tokens, parsed.comments, parsed.events);
        write_file(p, output);
    });
}

/// Return all Modelica files from the given directory
fn get_files_from_dir(dir: PathBuf) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let paths = fs::read_dir(&dir)
        .unwrap_or_else(|_| panic!("{}: error reading from a directory", dir.display()));
    paths
        .map(|e| e.unwrap().path())
        .map(|p| {
            if p.is_dir() {
                get_files_from_dir(p)
            } else if is_modelica(p.as_path()) {
                vec![p]
            } else {
                Vec::new()
            }
        })
        .for_each(|mut v| files.append(&mut v));

    files
}

/// Return `true` if the file is a Modelica file
fn is_modelica(f: &Path) -> bool {
    if let Some(suffix) = f.extension() {
        return suffix == "mo";
    }
    false
}

/// Return contents of the Modelica file
fn read_file(from: &Path) -> String {
    if !is_modelica(from) {
        panic!("{}: is not a Modelica file", from.display());
    }
    fs::read_to_string(from).unwrap_or_else(|_| panic!("{}: error reading a file", from.display()))
}

/// Write formatted code to a file
fn write_file(to: &Path, code: String) {
    fs::write(to, code).unwrap_or_else(|_| panic!("{}: error writing a file", to.display()));
}
