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
        let rna_sequence = self.sequence
            .replace("A", "U")
            .replace("T", "A")
            .replace("G", "X")
            .replace("C", "G")
            .replace("X", "C");
        RibonucleicAcid::new(rna_sequence.as_str())
    }
}
