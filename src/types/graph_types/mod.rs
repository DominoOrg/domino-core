use std::collections::{HashMap, VecDeque};
use graph::Graph;

use crate::types::domino_types::error::DominoError;

use super::graph_types::node::Node;

pub(crate) mod node;
pub(crate) mod graph;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
enum Orientation {
    #[default]
    Negative,
    Positive
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub(crate) struct Arc {
    pub source: Node,
    pub destination: Node,
    pub orientation: Option<Orientation>,
    pub position: Option<usize>
}