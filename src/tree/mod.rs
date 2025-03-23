use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fmt,
};

use node::Node;
use node_data::NodeData;

mod node;
mod node_data;

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
        while heap.len() > 1 {
            let left = match heap.pop() {
                Some(Reverse(left)) => left,
                None => return Err(HuffmanError("Could not get next node from heap".into())),
            };
            let right = match heap.pop() {
                Some(Reverse(right)) => right,
                None => return Err(HuffmanError("Could not get next node from heap".into())),
            };
            let combined_freq = left.data().freq() + right.data().freq();

            let parent = Node::node(NodeData::new('\0', combined_freq), left, right);
            heap.push(Reverse(parent));
        }

        let root = heap.pop().map(|Reverse(node)| node);
        Ok(HuffmanTree::new(root))
    }
}

pub(crate) fn freq_map(text: &str) -> HashMap<char, u64> {
    let freq_map: HashMap<char, u64> = text.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });
    freq_map
}

pub(crate) fn min_heap(text: &str) -> BinaryHeap<Reverse<Node>> {
    freq_map(text)
        .into_iter()
        .fold(BinaryHeap::<Reverse<Node>>::new(), |mut heap, pair| {
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
