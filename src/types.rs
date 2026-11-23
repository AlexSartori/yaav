use std::{collections::HashMap, hash::Hash, sync::LazyLock};

pub type MultiAlignment = Vec<Sequence>;

#[derive(Debug,Clone)]
pub struct Sequence {
    pub name: String,
    pub original_seq: Vec<char>,
    pub aa_seq: Option<Vec<char>>,
    uid: u64,
    version: u64
}

impl Sequence {
    pub fn new(uid: u64, id: &str, seq: Vec<char>) -> Self {
        Self {
            name: id.to_owned(),
            original_seq: seq,
            aa_seq: None,
            uid: uid,
            version: 0
        }
    }

    pub fn update_aa_translation(&mut self) {
        let mut aa_seq = Vec::new();

        let mut i = 0;
        while i < self.original_seq.len() {
            if i + 2 >= self.original_seq.len() {
                aa_seq.push('?');
            } else {
                let k: (char, char, char) = (
                    self.original_seq[i].to_ascii_uppercase(),
                    self.original_seq[i+1].to_ascii_uppercase(),
                    self.original_seq[i+2].to_ascii_uppercase()
                );
                aa_seq.push(*NT_TO_AA.get(&k).unwrap_or(&'?'));
            }
            i += 3;
        }

        self.aa_seq = Some(aa_seq);
    }
}

impl Hash for Sequence {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.uid.hash(state);
        self.version.hash(state);
    }
}


#[derive(Debug,Clone,PartialEq)]
pub enum SequenceEncoding {
    Nucleotidic,
    Aminoacidic
}

impl std::fmt::Display for SequenceEncoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            SequenceEncoding::Nucleotidic => "Nucleotidic",
            SequenceEncoding::Aminoacidic => "Aminoacidic"
        })
    }
}

static NT_TO_AA: LazyLock<HashMap<(char, char, char), char>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert(('A', 'A', 'A'), 'K');
    m.insert(('A', 'A', 'C'), 'N');
    m.insert(('A', 'A', 'G'), 'K');
    m.insert(('A', 'A', 'T'), 'N');
    m.insert(('A', 'C', 'A'), 'T');
    m.insert(('A', 'C', 'C'), 'T');
    m.insert(('A', 'C', 'G'), 'T');
    m.insert(('A', 'C', 'T'), 'T');
    m.insert(('A', 'G', 'A'), 'R');
    m.insert(('A', 'G', 'C'), 'S');
    m.insert(('A', 'G', 'G'), 'R');
    m.insert(('A', 'G', 'T'), 'S');
    m.insert(('A', 'T', 'A'), 'I');
    m.insert(('A', 'T', 'C'), 'I');
    m.insert(('A', 'T', 'G'), 'M');
    m.insert(('A', 'T', 'T'), 'I');
    m.insert(('C', 'A', 'A'), 'Q');
    m.insert(('C', 'A', 'C'), 'H');
    m.insert(('C', 'A', 'G'), 'Q');
    m.insert(('C', 'A', 'T'), 'H');
    m.insert(('C', 'C', 'A'), 'P');
    m.insert(('C', 'C', 'C'), 'P');
    m.insert(('C', 'C', 'G'), 'P');
    m.insert(('C', 'C', 'T'), 'P');
    m.insert(('C', 'G', 'A'), 'R');
    m.insert(('C', 'G', 'C'), 'R');
    m.insert(('C', 'G', 'G'), 'R');
    m.insert(('C', 'G', 'T'), 'R');
    m.insert(('C', 'T', 'A'), 'L');
    m.insert(('C', 'T', 'C'), 'L');
    m.insert(('C', 'T', 'G'), 'L');
    m.insert(('C', 'T', 'T'), 'L');
    m.insert(('G', 'A', 'A'), 'E');
    m.insert(('G', 'A', 'C'), 'D');
    m.insert(('G', 'A', 'G'), 'E');
    m.insert(('G', 'A', 'T'), 'D');
    m.insert(('G', 'C', 'A'), 'A');
    m.insert(('G', 'C', 'C'), 'A');
    m.insert(('G', 'C', 'G'), 'A');
    m.insert(('G', 'C', 'T'), 'A');
    m.insert(('G', 'G', 'A'), 'G');
    m.insert(('G', 'G', 'C'), 'G');
    m.insert(('G', 'G', 'G'), 'G');
    m.insert(('G', 'G', 'T'), 'G');
    m.insert(('G', 'T', 'A'), 'V');
    m.insert(('G', 'T', 'C'), 'V');
    m.insert(('G', 'T', 'G'), 'V');
    m.insert(('G', 'T', 'T'), 'V');
    m.insert(('T', 'A', 'A'), '*');
    m.insert(('T', 'A', 'C'), 'Y');
    m.insert(('T', 'A', 'G'), '*');
    m.insert(('T', 'A', 'T'), 'Y');
    m.insert(('T', 'C', 'A'), 'S');
    m.insert(('T', 'C', 'C'), 'S');
    m.insert(('T', 'C', 'G'), 'S');
    m.insert(('T', 'C', 'T'), 'S');
    m.insert(('T', 'G', 'A'), '*');
    m.insert(('T', 'G', 'C'), 'C');
    m.insert(('T', 'G', 'G'), 'W');
    m.insert(('T', 'G', 'T'), 'C');
    m.insert(('T', 'T', 'A'), 'L');
    m.insert(('T', 'T', 'C'), 'F');
    m.insert(('T', 'T', 'G'), 'L');
    m.insert(('T', 'T', 'T'), 'F');
    m.insert(('-', '-', '-'), '-');
    m
});

// m.insert('A', &[('G', 'C', 'A'), ('G', 'C', 'C'), ('G', 'C', 'G'), ('G', 'C', 'T')]);
// m.insert('C', &[('T', 'G', 'C'), ('T', 'G', 'T')]);
// m.insert('D', &[('G', 'A', 'C'), ('G', 'A', 'T')]);
// m.insert('E', &[('G', 'A', 'A'), ('G', 'A', 'G')]);
// m.insert('F', &[('T', 'T', 'C'), ('T', 'T', 'T')]);
// m.insert('G', &[('G', 'G', 'A'), ('G', 'G', 'C'), ('G', 'G', 'G'), ('G', 'G', 'T')]);
// m.insert('H', &[('C', 'A', 'C'), ('C', 'A', 'T')]);
// m.insert('I', &[('A', 'T', 'A'), ('A', 'T', 'C'), ('A', 'T', 'T')]);
// m.insert('K', &[('A', 'A', 'A'), ('A', 'A', 'G')]);
// m.insert('L', &[('C', 'T', 'A'), ('C', 'T', 'C'), ('C', 'T', 'G'), ('C', 'T', 'T'), ('T', 'T', 'A'), ('T', 'T', 'G')]);
// m.insert('M', &[('A', 'T', 'G')]);
// m.insert('N', &[('A', 'A', 'C'), ('A', 'A', 'T')]);
// m.insert('P', &[('C', 'C', 'A'), ('C', 'C', 'C'), ('C', 'C', 'G'), ('C', 'C', 'T')]);
// m.insert('Q', &[('C', 'A', 'A'), ('C', 'A', 'G')]);
// m.insert('R', &[('A', 'G', 'A'), ('A', 'G', 'G'), ('C', 'G', 'A'), ('C', 'G', 'C'), ('C', 'G', 'G'), ('C', 'G', 'T')]);
// m.insert('S', &[('A', 'G', 'C'), ('A', 'G', 'T'), ('T', 'C', 'A'), ('T', 'C', 'C'), ('T', 'C', 'G'), ('T', 'C', 'T')]);
// m.insert('T', &[('A', 'C', 'A'), ('A', 'C', 'C'), ('A', 'C', 'G'), ('A', 'C', 'T')]);
// m.insert('V', &[('G', 'T', 'A'), ('G', 'T', 'C'), ('G', 'T', 'G'), ('G', 'T', 'T')]);
// m.insert('W', &[('T', 'G', 'G')]);
// m.insert('Y', &[('T', 'A', 'C'), ('T', 'A', 'T')]);
// m.insert('*', &[('T', 'A', 'A'), ('T', 'A', 'G'), ('T', 'G', 'A')]);