pub type Nucleotide = char;
pub type MultiAlignment = Vec<Sequence>;

#[derive(Debug,Clone)]
pub struct Sequence {
    pub id: String,
    pub seq: Vec<Nucleotide>
}

