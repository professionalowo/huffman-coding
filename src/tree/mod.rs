use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fmt,
};

use node::Node;
use node_data::NodeData;

mod node;
mod node_data;

pub type MinHeap<T> = BinaryHeap<Reverse<T>>;

#[derive(Debug, Clone)]
pub struct HuffmanError(pub String);

impl fmt::Display for HuffmanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct HuffmanTree {
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

    pub fn from_text(text: &str) -> Result<Self, HuffmanError> {
        let mut heap = min_heap(text);
        Self::build_tree(&mut heap)
    }
    /**
     * Build a Huffman tree from a min heap
     */
    fn build_tree(heap: &mut MinHeap<Node>) -> Result<Self, HuffmanError> {
        while heap.len() > 1 {
            let parent = Reverse(Self::combine_nodes(heap)?);
            heap.push(parent);
        }

        let root = heap.pop().map(|Reverse(node)| node);
        Ok(HuffmanTree::new(root))
    }

    /**
     * pops the next two elements from the heap and creates a new internal node that has the first and second element as children
     */
    fn combine_nodes(heap: &mut MinHeap<Node>) -> Result<Node, HuffmanError> {
        let left = get_next_node(heap)?;
        let right = get_next_node(heap)?;
        let data = NodeData::internal(left.data().freq() + right.data().freq());
        Ok(Node::node(data, left, right))
    }
}

fn get_next_node(heap: &mut MinHeap<Node>) -> Result<Node, HuffmanError> {
    match heap.pop() {
        Some(Reverse(left)) => Ok(left),
        None => Err(HuffmanError("Could not get next node from heap".into())),
    }
}

pub(crate) fn freq_map(text: &str) -> HashMap<char, u64> {
    text.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    })
}

pub(crate) fn min_heap(text: &str) -> MinHeap<Node> {
    freq_map(text)
        .into_iter()
        .fold(MinHeap::new(), |mut heap, pair| {
            heap.push(Reverse(Node::leaf(pair.into())));
            heap
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn freq_map_test() {
        let result = freq_map("aab");
        assert_eq!(result, HashMap::from([('a', 2), ('b', 1)]));
    }

    #[test]
    fn max_heap_test() {
        let result = min_heap("aab");

        let expected = BinaryHeap::from([
            Reverse(Node::leaf(NodeData::new('a', 2))),
            Reverse(Node::leaf(NodeData::new('b', 1))),
        ]);
        assert!(result.iter().eq(expected.iter()))
    }

    #[test]
    fn huffman_tree_from_text_test() {
        let text = "aabbbc"; // Example text
        let tree = HuffmanTree::from_text(text);
        println!("{:#?}", tree);
        // Ensure the tree is not empty
        assert!(tree.is_ok(), "Tree root should not be None");

        let root = tree.expect("tree has to be ok").root.unwrap();

        // Check if the root frequency equals the sum of character frequencies
        let expected_freq = text.chars().count() as u64; // Total character count
        assert_eq!(
            root.data().freq(),
            &expected_freq,
            "Root frequency should match total character count"
        );

        // Optional: Check if the tree structure is valid (left and right exist)
        if let Node::Node { left, right, .. } = &root {
            assert!(
                left.data().freq() < root.data().freq(),
                "Left child should have lower frequency than root"
            );
            assert!(
                right.data().freq() < root.data().freq(),
                "Right child should have lower frequency than root"
            );
        }
    }
}
