use moparse::TokenCollection;

use crate::markers::Marker;


pub struct Printer<'a> {
    indent: usize,
    tokens: &'a TokenCollection,
    markers: &'a Vec<Marker>,
    formatted: String,
}

impl<'a> Printer<'a> {

    pub fn new(tokens: &'a TokenCollection, markers: &'a Vec<Marker>) -> Self {
        Printer { indent: 0, tokens, markers, formatted: String::new() }
    }

    fn print_marker(&mut self, m: &Marker) -> String {
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
            Marker::Token(i) | Marker::Comment(i) => self.tokens.get_item(*i).unwrap().text.clone(),
            _ => {
                let mut out = String::from("\n");;
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

    pub fn pretty_print(&mut self) {
        for marker in self.markers {
            let s = self.print_marker(marker);
            self.formatted += s.as_str();
        }
    }
}
