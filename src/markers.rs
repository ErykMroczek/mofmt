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
    pub markers: Vec<Marker>,
}

impl MarkerCollector {
    pub fn new() -> Self {
        MarkerCollector {
            markers: Vec::new(),
        }
    }

    pub fn push(&mut self, m: Marker) {
        match m {
            Marker::Space => {
                if self.markers.len() == 0 || *self.markers.last().unwrap() >= Marker::Space {
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
