use crate::parser::ModelicaCST;

use super::formatting::Marker;

pub fn print(cst: &ModelicaCST, markers: Vec<Marker>) -> String {
    let mut printer = Printer::new();
    let formatted: Vec<String> = markers
        .into_iter()
        .filter_map(|m| printer.print_marker(cst, m))
        .collect();
    formatted.join("")
}

struct Printer {
    indent: usize,
}

impl Printer {
    fn new() -> Self {
        Printer { indent: 0 }
    }

    fn print_marker(&mut self, cst: &ModelicaCST, m: Marker) -> Option<String> {
        const INDENT: &str = "  ";
        const EOL: &str = if cfg!(windows) { "\r\n" } else { "\n" };
        match m {
            Marker::Space => Some(String::from(" ")),
            Marker::Indent => {
                self.indent += 1;
                None
            }
            Marker::Dedent => {
                self.indent -= 1;
                None
            }
            Marker::Token(tok) => Some(String::from(cst.tokens().text(tok))),
            _ => {
                let mut out = String::from(EOL);
                if m == Marker::Blank {
                    out += EOL;
                }
                (0..self.indent).for_each(|_| out += INDENT);
                Some(out)
            }
        }
    }
}
