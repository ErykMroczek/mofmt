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
        match contents {
            Ok(source) => {
                let parsed = parse(&source, SyntaxKind::StoredDefinition);
                if !parsed.errors.is_empty() {
                    let messages: Vec<String> = parsed
                        .errors
                        .iter()
                        .map(|e| format!("{}:{}", p.display(), e))
                        .collect();
                    println!(
                        "Syntax errors detected (mofmt won't touch this file):\n{}",
                        messages.join("\n")
                    );
                } else {
                    let output = pretty_print(parsed.tokens, parsed.comments, parsed.events);
                    write_file(p, output);
                }
            }
            Err(e) => println!("{}: error: {}", p.display(), e),
        }
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
fn read_file(from: &Path) -> Result<String, String> {
    if !is_modelica(from) {
        return Err(format!("{} is not a Modelica file", from.display()));
    }
    match fs::read_to_string(from) {
        Ok(s) => Ok(s),
        Err(e) => Err(e.to_string()),
    }
}

/// Write formatted code to a file
fn write_file(to: &Path, code: String) {
    fs::write(to, code).unwrap_or_else(|_| panic!("{}: error writing a file", to.display()));
}
