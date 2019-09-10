#[derive(Debug, PartialEq)]
pub struct DNA {
    sequence: String,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    sequence: String,
}

const VALID_DNA_NUCLEOTIDES: &[char] = &['G', 'C', 'T', 'A'];
const VALID_RNA_NUCLEOTIDES: &[char] = &['C', 'G', 'A', 'U'];

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        if let Some(invalid) = dna.chars().find(|c| !VALID_DNA_NUCLEOTIDES.contains(c)) {
            return Err(dna.chars().position(|c| c == invalid).unwrap());
        }
        Ok(DNA {
            sequence: dna.to_owned(),
        })
    }

    pub fn into_rna(self) -> RNA {
        let converted_sequence: String = self
            .sequence
            .chars()
            .map(|c| {
                let index = VALID_DNA_NUCLEOTIDES.iter().position(|&n| n == c).unwrap();
                VALID_RNA_NUCLEOTIDES[index]
            })
            .collect();
        RNA::new(&converted_sequence).unwrap()
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        if let Some(invalid) = rna.chars().find(|c| !VALID_RNA_NUCLEOTIDES.contains(c)) {
            return Err(rna.chars().position(|c| c == invalid).unwrap());
        }
        Ok(RNA {
            sequence: rna.to_owned(),
        })
    }
}
