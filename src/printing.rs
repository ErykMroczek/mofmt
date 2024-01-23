use moparse::TokenCollection;

use crate::markers::Marker;

pub fn pretty_print(tokens: &TokenCollection, markers: Vec<Marker>) -> String {
    let mut printer = Printer::new();
    let formatted: Vec<String> = markers
        .into_iter()
        .filter_map(|m| printer.print_marker(m, tokens))
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

    fn print_marker(&mut self, m: Marker, tokens: &TokenCollection) -> Option<String> {
        const INDENT: &str = "  ";
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
            Marker::Token(i) | Marker::Comment(i) => Some(tokens.get_item(i).unwrap().text.clone()),
            _ => {
                let mut out = String::from("\n");
                if m == Marker::Blank {
                    out += "\n";
                }
                (0..self.indent).for_each(|_| out += INDENT);
                Some(out)
            }
        }
    }
}
