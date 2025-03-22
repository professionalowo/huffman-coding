use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use node::Node;
use node_data::NodeData;

mod node;
mod node_data;

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

    pub fn from_text(text: &str) -> Self {
        let min_heap = Reverse(max_heap(text));
        println!("{:#?}", min_heap);
        HuffmanTree::empty()
    }
}

pub(crate) fn freq_map(text: &str) -> HashMap<char, u64> {
    let freq_map: HashMap<char, u64> = text.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });
    freq_map
}

pub(crate) fn max_heap(text: &str) -> BinaryHeap<NodeData> {
    freq_map(text)
        .into_iter()
        .fold(BinaryHeap::<NodeData>::new(), |mut heap, pair| {
            heap.push(pair.into());
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
        let result = max_heap("aab");

        let expected = BinaryHeap::from([NodeData::new('a', 2), NodeData::new('b', 1)]);
        assert!(result.iter().eq(expected.iter()))
    }
}
