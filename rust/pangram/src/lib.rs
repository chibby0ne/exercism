use std::collections::HashSet;
/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let set: HashSet<u8> = sentence
        .to_ascii_uppercase()
        .chars()
        .filter(|x| x.is_ascii())
        .fold(HashSet::new(), |mut acc, c| {
            acc.insert(c as u8);
            acc
        });

    for c in b'A'..=b'Z' {
        if !set.contains(&c) {
            return false;
        }
    }
    true
}
