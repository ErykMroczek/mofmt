use moparse::TokenCollection;

use crate::markers::Marker;

pub fn pretty_print(tokens: &TokenCollection, markers: &Vec<Marker>) -> String {
    let mut printer = Printer::new();
    let mut formatted = String::new();
    for marker in markers {
        let s = printer.print_marker(marker, tokens);
        formatted += s.as_str();
    }
    formatted
}

struct Printer {
    indent: usize,
}

impl Printer {
    fn new() -> Self {
        Printer { indent: 0 }
    }

    fn print_marker(&mut self, m: &Marker, tokens: &TokenCollection) -> String {
        const INDENT: &str = "  ";
        match m {
            Marker::Space => String::from("  "),
            Marker::Ignore => String::new(),
            Marker::Indent => {
                self.indent += 1;
                String::new()
            }
            Marker::Dedent => {
                self.indent -= 1;
                String::new()
            }
            Marker::Token(i) | Marker::Comment(i) => tokens.get_item(*i).unwrap().text.clone(),
            _ => {
                let mut out = String::from("\n");
                if m == &Marker::Blank {
                    out += "\n";
                }
                for i in 1..self.indent {
                    out += INDENT
                }
                out
            }
        }
    }
}
