use super::graph_types::node::Node;

pub(crate) mod node;
pub(crate) mod graph;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub(crate) struct Arc {
    pub source: Node,
    pub destination: Node,
    pub position: Option<usize>
}