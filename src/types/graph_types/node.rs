use crate::types::domino_types::error::DominoError;

use super::Arc;

#[derive(Debug, Clone, Eq, Hash)]
pub(crate) enum Node {
    Regular(i32),
    Auxiliary((i32, i32, usize))
}

impl Default for Node {
    fn default() -> Self {
        Node::Regular(0)
    }
}

impl From<i32> for Node {
    fn from(value: i32) -> Self {
        Node::Regular(value)
    }
}

impl From<(i32, i32, usize)> for Node {
    fn from(value: (i32, i32, usize)) -> Self {
        Node::Auxiliary(value)
    }
}

impl TryFrom<Arc> for Node {
    type Error = DominoError;

    fn try_from(value: Arc) -> Result<Self, Self::Error> {
        if value.orientation.is_some() { return Err(DominoError::UnsolvableGraph("Invalid node conversion from oriented edge into a node of the auxiliary graph".to_string()))}
        Ok(Node::Auxiliary((value.source.try_into().unwrap(), value.destination.try_into().unwrap(), value.position.unwrap())))
    }
}
impl TryInto<i32> for Node {
    type Error = DominoError;

    fn try_into(self) -> Result<i32, Self::Error> {
        match self {
            Node::Regular(node) => Ok(node),
            _ => Err(DominoError::UnsolvableGraph("Invalid node conversion from auxiliary graph into regular one".to_string()))
        }
    }
}

impl TryInto<(i32, i32, usize)> for Node {
    type Error = DominoError;

    fn try_into(self) -> Result<(i32, i32, usize), Self::Error> {
        match self {
            Node::Auxiliary(node) => Ok(node),
            _ => Err(DominoError::UnsolvableGraph("Invalid node conversion from regular graph into auxiliary one".to_string()))
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Node::Regular(a), Node::Regular(b)) => a == b,
            (Node::Auxiliary(a), Node::Auxiliary(b)) => {
                a == b
            },
            _ => false
        }
    }
}

impl Node {
    pub fn try_position(&self) -> Result<usize, DominoError> {
        match self {
            Node::Regular(_) => Err(DominoError::UnsolvableGraph("Invalid node conversion from regular one into position".to_string()))?,
            Node::Auxiliary(node) => Ok(node.2)
        }
    }

    pub fn try_tile(&self) -> Result<(i32, i32), DominoError> {
        match self {
            Node::Regular(_) => Err(DominoError::UnsolvableGraph("Invalid node conversion from regular one into tile".to_string()))?,
            Node::Auxiliary(node) => Ok((node.0, node.1))
        }
    }
}
