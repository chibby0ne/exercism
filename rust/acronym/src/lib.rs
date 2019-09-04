pub fn abbreviate(phrase: &str) -> String {
    let mut new_acronym = String::new();
    if phrase.is_empty() {
        return new_acronym;
    }
    // Remove the - and the : from the phrase and split it by whitespaces into a slice
    for slice in phrase
        .replace("-", " ")
        .replace(":", " ")
        .split_whitespace()
    {
        // Insert first char as uppercase
        new_acronym.push_str(slice.get(0..1).unwrap().to_uppercase().as_str());
        // If all the chars are uppercase in this slice of the whole word, then only insert the
        // first char of this slice
        if slice.chars().all(|c| c.is_uppercase()) {
            continue;
        }
        // Find all the uppercase letters in this slice
        let uppercases = slice
            .chars()
            .enumerate()
            .filter(|(count, v)| *count != 0 as usize && v.is_uppercase())
            .map(|(_, v)| v)
            .collect::<Vec<char>>();
        // append all the uppercase letters if any
        for c in uppercases {
            new_acronym.push(c);
        }
    }
    new_acronym
}
