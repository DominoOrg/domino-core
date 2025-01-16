use crate::types::error::DominoError;

#[derive(Debug, Clone, Eq, Hash)]
pub(crate) enum Node {
    Regular(i32),
    Auxiliary((i32, i32))
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

impl From<(i32, i32)> for Node {
    fn from(value: (i32, i32)) -> Self {
        Node::Auxiliary(value)
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

impl TryInto<(i32, i32)> for Node {
    type Error = DominoError;

    fn try_into(self) -> Result<(i32, i32), Self::Error> {
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
