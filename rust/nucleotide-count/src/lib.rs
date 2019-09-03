use std::collections::HashMap;

static VALID_NUCLEOTIDES: &'static [char] = &['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    // Check for empty dna, if empty return Ok(0)
    if dna.len() == 0 {
        return Ok(0);
    }
    // If the nucleotide we need to look for is not a valid one, then we return the nucleotide as an Err variant of Result
    if VALID_NUCLEOTIDES
        .iter()
        .find(|&&c| c == nucleotide)
        .is_none()
    {
        return Err(nucleotide);
    }
    // If the the dna contains an invalid nucleotide, return that invalid nucleotide as an Err variant of Result
    if let Some(c) = dna
        .chars()
        .find(|&c| VALID_NUCLEOTIDES.iter().all(|&v| v != c))
    {
        return Err(c);
    }
    // Wrap the count of ocurrences of the nucleotide in dna in a Ok variant and return it
    Ok(dna.chars().filter(|&c| c == nucleotide).count())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut map: HashMap<char, usize> = HashMap::new();
    // For each valid nucleotide count the ocurrences of it in the dna str, and insert it into a
    // map of the nucleotide with its ocurrences' count if it was was a dna str, or return an Err
    // with the invalid nucleotide inside the dna str
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
