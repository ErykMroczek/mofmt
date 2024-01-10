#[derive(PartialEq, PartialOrd)]
pub enum Marker {
    Token(usize),
    Comment(usize),
    Indent,
    Dedent,
    Space,
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
                if [Marker::Indent, Marker::Dedent].contains(self.markers.last().unwrap())
                    && *self.markers.get(self.markers.len() - 2).unwrap() >= Marker::Blank
                {
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

    pub fn cache_tail(&mut self) -> Vec<Marker> {
        let mut tail = Vec::new();
        loop {
            let last = self.markers.last();
            if last.is_none() {
                break;
            }
            if *last.unwrap() < Marker::Indent {
                break;
            }
            if *last.unwrap() != Marker::Space {
                tail.push(self.markers.pop().unwrap());
            }
        }
        tail.reverse();
        tail
    }

    pub fn append(&mut self, markers: &mut Vec<Marker>) {
        self.markers.append(markers);
    }
}
