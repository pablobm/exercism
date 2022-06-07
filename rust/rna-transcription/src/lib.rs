#[derive(PartialEq)]
#[derive(Debug)]
pub struct Rna {
    sequence: String,
}

fn is_not_valid_rna_nucleotide(nucleotide: char) -> bool {
    !is_valid_rna_nucleotide(nucleotide)
}

fn is_not_valid_dna_nucleotide(nucleotide: char) -> bool {
    !is_valid_dna_nucleotide(nucleotide)
}

fn is_valid_rna_nucleotide(nucleotide: char) -> bool {
    match nucleotide {
        'A' | 'U' | 'G' | 'C' => true,
        _ => false
    }
}

fn is_valid_dna_nucleotide(nucleotide: char) -> bool {
    match nucleotide {
        'A' | 'T' | 'G' | 'C' => true,
        _ => false
    }
}

impl Rna {
    pub fn new(sequence: &str) -> Result<Rna, usize> {
        let maybe_index = sequence.find(is_not_valid_rna_nucleotide);
        match maybe_index {
            None => (),
            Some(index) => return Err(index)
        }

        let rna = Rna {
            sequence: sequence.to_string(),
        };
        Ok(rna)
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Dna {
    sequence: String,
}

impl Dna {
    pub fn new(sequence: &str) -> Result<Dna, usize> {
        let maybe_index = sequence.find(is_not_valid_dna_nucleotide);
        match maybe_index {
            None => (),
            Some(index) => return Err(index)
        }

        let dna = Dna {
            sequence: sequence.to_string(),
        };
        Ok(dna)
    }

    pub fn into_rna(&self) -> Rna {
        let rna_sequence = self.sequence.chars()
            .map(transcribe)
            .collect::<String>();
        Rna::new(&rna_sequence).unwrap()
    }
}

fn transcribe(nucleotide: char) -> char {
    match nucleotide {
        'A' => 'U',
        'T' => 'A',
        'G' => 'C',
        'C' => 'G',
        _ => panic!("I don't know how to transcribe '{}'", nucleotide),
    }
}
