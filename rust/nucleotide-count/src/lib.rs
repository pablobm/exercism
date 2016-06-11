use std::collections::HashMap;

pub fn count(nucleotide: char, sequence: &str) -> usize {
    sequence.chars().filter(|&ch| ch == nucleotide).count()
}

pub fn nucleotide_counts(sequence: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    map.insert('A', count('A', sequence));
    map.insert('T', count('T', sequence));
    map.insert('G', count('G', sequence));
    map.insert('C', count('C', sequence));
    map
}
