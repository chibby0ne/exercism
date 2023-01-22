use std::collections::HashSet;
/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let set: HashSet<_> = sentence
        .to_ascii_uppercase()
        .chars()
        .filter(|x| x.is_ascii_uppercase())
        .collect();
    set.len() == 26
}
