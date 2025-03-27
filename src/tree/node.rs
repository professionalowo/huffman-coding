use super::node_data::NodeData;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub enum Node {
    Leaf(NodeData),
    Node {
        data: NodeData,
        left: Box<Self>,
        right: Box<Self>,
    },
}

impl Node {
    pub fn data(&self) -> &NodeData {
        match self {
            Self::Leaf(data) | Self::Node { data, .. } => data,
        }
    }

    pub fn leaf(data: NodeData) -> Self {
        Self::Leaf(data)
    }

    pub fn node(data: NodeData, left: Self, right: Self) -> Self {
        Self::Node {
            data,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /**
     * Creates a new internal Node with self as left and other as right
     */
    pub fn combine(self, other: Self) -> Self {
        let data = NodeData::internal(self.data().freq() + other.data().freq());
        Self::Node { data, left: Box::new(self), right: Box::new(other) }
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
