use mofmt::pretty_print;
use moparse::{parse, SyntaxKind};
use std::io::{stdout, Write};
use std::path::{Path, PathBuf};
use std::{env, fs};

const VERSION: &str = "0.5.0";

const HELP: &str = r#"
mofmt: Modelica code formatter

Usage: mofmt [OPTIONS] <PATHS>

Options:
-h, --help: display this message and exit
-v, --version: display a version number and exit
--check: run mofmt in check mode (without modifying the file)
"#;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Missing PATHS arguments.\n{}", HELP);
        std::process::exit(1);
    } else if ["-h", "--help"].contains(&args[1].as_str()) {
        println!("{}", HELP);
        std::process::exit(0);
    } else if ["-v", "--version"].contains(&args[1].as_str()) {
        println!("mofmt, {}", VERSION);
        std::process::exit(0);
    } else if args[1].as_str() == "--check" {
        if args.len() < 3 {
            eprintln!("Missing PATHS arguments.\n{}", HELP);
            std::process::exit(1);
        }
        format_files(&args[2..], true);
    } else if args[1].starts_with('-') {
        eprintln!("Unrecognized option: '{}'.\n{}", args[1], HELP);
        std::process::exit(1);
    } else {
        format_files(&args[1..], false);
    }
}

/// Format files specified in the argument list
fn format_files(args: &[String], check: bool) {
    let mut code = 0;
    let mut files = Vec::new();
    let mut lock = stdout().lock();
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
        let name = p.display();
        match contents {
            Ok(source) => {
                let parsed = parse(name.to_string().as_str(), &source, SyntaxKind::StoredDefinition);
                if !parsed.errors.is_empty() {
                    writeln!(lock, "Syntax errors detected:\n{}", parsed.errors.join("\n")).unwrap();
                    code = 1;
                } else {
                    let output = pretty_print(parsed.tokens, parsed.comments, parsed.events);
                    if check {
                        if output != source {
                            code = 1;
                            writeln!(lock, "{}: check failed", name).unwrap();
                        } else {
                            writeln!(lock, "{}: check passed", name).unwrap();
                        }
                    } else {
                        write_file(p, output);
                    }
                }
            }
            Err(e) => {
                eprintln!("{}: error: {}", name, e);
                code = 1;
            }
        }
    });
    std::process::exit(code);
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
