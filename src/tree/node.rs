use super::node_data::NodeData;

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
            Self::Leaf(data) | Self::Node { data, .. } => data
        }
    } 
}
