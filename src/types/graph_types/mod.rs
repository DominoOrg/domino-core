mod graph;

pub use graph::Graph;
pub type Node = i32;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Arc {
    pub source: Node,
    pub destination: Node,
    pub position: Option<usize>,
}
