use std::collections::HashMap;

const VALID_NUCLEOTIDES: &str = "ATGC";

pub fn count(nucleotide: char, sequence: &str) -> Result<usize, char> {
    if !VALID_NUCLEOTIDES.contains(nucleotide) {
        return Err(nucleotide)
    }

    let mut count = 0;

    for c in sequence.chars() {
        if !VALID_NUCLEOTIDES.contains(c) {
            return Err(c)
        }
        if c == nucleotide {
            count += 1;
        }
    }
    Ok(count)
}

pub fn nucleotide_counts(sequence: &str) -> Result<HashMap<char, usize>, char> {
    let mut map = HashMap::new();
    for nuc in VALID_NUCLEOTIDES.chars() {
        let result = count(nuc, sequence);
        match result {
            Ok(number) => {
                map.insert(nuc, number);
                ()
            }
            Err(err) => {
                return Err(err)
            }
        }
    }
    Ok(map)
}
