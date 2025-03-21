use std::cmp::Ordering;

#[derive(Debug)]
pub(crate) struct NodeData {
    data: char,
    freq: u64,
}

impl NodeData {
    pub fn new(data: char, freq: u64) -> Self {
        Self { data, freq }
    }

    pub fn data(&self) -> &char {
        &self.data
    }

    pub fn freq(&self) -> &u64 {
        &self.freq
    }
}

impl Ord for NodeData {
    fn cmp(&self, other: &Self) -> Ordering {
        self.freq().cmp(other.freq())
    }
}

impl PartialOrd for NodeData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for NodeData {}

impl PartialEq for NodeData {
    fn eq(&self, other: &Self) -> bool {
        self.freq() == other.freq()
    }
}
