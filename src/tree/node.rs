use super::node_data::NodeData;
use std::cmp::Ordering;

#[derive(Debug)]
pub(crate) enum Node {
    Leaf(NodeData),
    Node {
        data: NodeData,
        left: Box<Node>,
        right: Box<Node>,
    },
}

impl Node {
    pub fn data(&self) -> &NodeData {
        match self {
            Self::Leaf(data) | Self::Node { data, .. } => data,
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.data().cmp(other.data())
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.data() == other.data()
    }
}
