use node::Node;

mod node;
mod node_data;

#[derive(Debug)]
pub(crate) struct HuffmanTree {
    root: Option<Node>,
}

impl HuffmanTree {
    pub fn new(root: Option<Node>) -> Self {
        Self { root }
    }

    pub fn new_root(root: Node) -> Self {
        Self { root: Some(root) }
    }

    pub fn empty() -> Self {
        Self { root: None }
    }

    pub fn root(&self) -> Option<&Node> {
        self.root.as_ref()
    }
}
