pub(crate) mod graph;

pub(crate) type Node = i32;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub(crate) struct Arc {
    pub source: Node,
    pub destination: Node,
    pub position: Option<usize>
}