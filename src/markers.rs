
#[derive(PartialEq, PartialOrd)]
pub enum Marker {
    Token(usize),
    Comment(usize),
    Space,
    Indent,
    Dedent,
    Ignore,
    Blank,
    Break,
    Wrap,
}

pub struct MarkerCollector {
    markers: Vec<Marker>,
}

impl MarkerCollector {

    pub fn new() -> Self {
        MarkerCollector { markers: Vec::new() }
    }

    pub fn push(&mut self, m: Marker) {
        match m {
            Marker::Space => {
                if self.markers.len() == 0 {
                    return;
                } else if [Marker::Ignore, Marker::Space, Marker::Indent, Marker::Dedent].contains(&self.markers.last().unwrap()) {
                    return;
                }
            }
            Marker::Blank => {
                // Remove preceding break
                if *self.markers.last().unwrap() >= Marker::Blank {
                    self.markers.pop();
                }
            }
            Marker::Break => {
                // Do not add unnecessary breaks
                if *self.markers.last().unwrap() >= Marker::Blank {
                    return;
                }
            }
            _ => (),
        }
        self.markers.push(m);
    }

}
