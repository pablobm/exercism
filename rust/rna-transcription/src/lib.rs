#[derive(PartialEq)]
#[derive(Debug)]
pub struct RibonucleicAcid {
    sequence: String,
}

impl RibonucleicAcid {
    pub fn new(sequence: &str) -> RibonucleicAcid {
        RibonucleicAcid {
            sequence: sequence.to_string(),
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct DeoxyribonucleicAcid {
    sequence: String,
}

impl DeoxyribonucleicAcid {
    pub fn new(sequence: &str) -> DeoxyribonucleicAcid {
        DeoxyribonucleicAcid {
            sequence: sequence.to_string(),
        }
    }

    pub fn to_rna(&self) -> RibonucleicAcid {
        let rna_sequence = self.sequence.chars()
            .map(transcribe)
            .collect::<String>();
        RibonucleicAcid::new(&rna_sequence)
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