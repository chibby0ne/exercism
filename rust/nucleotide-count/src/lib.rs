use std::collections::HashMap;

static VALID_NUCLEOTIDES: &'static [char] = &['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if dna.len() == 0 {
        return Ok(0);
    }
    if let None = VALID_NUCLEOTIDES.iter().find(|&&c| c == nucleotide) {
        return Err(nucleotide);
    }
    if let Some(c) = dna
        .chars()
        .find(|&c| VALID_NUCLEOTIDES.iter().all(|&v| v != c))
    {
        return Err(c);
    }
    Ok(dna.chars().filter(|&c| c == nucleotide).count())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut map: HashMap<char, usize> = HashMap::new();
    for &c in VALID_NUCLEOTIDES {
        match count(c, dna) {
            Ok(num) => {
                map.insert(c, num);
            }
            Err(wrong_char) => {
                return Err(wrong_char);
            }
        }
    }
    Ok(map)
}
