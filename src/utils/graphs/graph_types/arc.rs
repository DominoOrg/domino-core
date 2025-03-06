pub type Node = i32;

#[derive(Debug, Default, Eq, Clone, Hash)]
pub struct Arc {
    pub source: Node,
    pub destination: Node,
    pub position: Option<usize>,
}

impl PartialEq for Arc {
    fn eq(&self, other: &Self) -> bool {
        (self.source == other.source && self.destination == other.destination)
            || (self.source == other.destination && self.destination == other.source)
    }
}

impl From<(Node, Node)> for Arc {
    fn from(value: (Node, Node)) -> Self {
        Arc {
            source: value.0,
            destination: value.1,
            position: None,
        }
    }
}
