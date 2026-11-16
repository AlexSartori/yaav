use std::{hash::Hash, sync::Arc};

pub type Nucleotide = char;
pub type MultiAlignment = Vec<Arc<Sequence>>;

#[derive(Debug,Clone)]
pub struct Sequence {
    pub id: String,
    pub seq: Vec<Nucleotide>,
    version: u64
}

impl Sequence {
    pub fn new(id: &str, seq: Vec<char>) -> Self {
        Self {
            id: id.to_owned(),
            seq: seq,
            version: 0
        }
    }
}

impl Hash for Sequence {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.version.hash(state);
    }
}

