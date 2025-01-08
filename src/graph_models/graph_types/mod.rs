use std::ops::Neg;

pub mod aux_graph;
pub mod di_graph;
pub mod graph;
pub(crate) mod helpers;
pub mod pog_graph;
pub mod regular_graph;
pub mod under_graph;

// Assuming GraphNode is a type alias for the graph's node type
type GraphNode = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Orientation {
    Positive = 1,
    Zero = 0,
    Negative = -1,
}

impl Neg for Orientation {
    type Output = Orientation;

    fn neg(self) -> Self::Output {
        match self {
            Orientation::Negative => Orientation::Positive,
            Orientation::Positive => Orientation::Negative,
            Orientation::Zero => Orientation::Zero,
        }
    }
}
