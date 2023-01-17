use std::collections::HashSet;

// Time complexity: O(N log N) -> sorts
// Space complexity: O(word_length + possible_length)
fn is_anagram<'a>(word: &'a str, possible: &'a str) -> Option<&'a str> {
    let word_lowercase = word.to_lowercase();
    let possible_lowercase = possible.to_lowercase();
    if word_lowercase == possible_lowercase || word_lowercase.len() != possible_lowercase.len() {
        return None;
    }

    let mut vec_word: Vec<char> = word_lowercase.chars().collect();
    let mut vec_possible: Vec<char> = possible_lowercase.chars().collect();

    vec_word.sort();
    vec_possible.sort();

    if vec_word == vec_possible {
        Some(possible)
    } else {
        None
    }
}

// Time complexity: O(possible_anagrams_length * O(word_length + MAX(possible_anagrams_length))
// Space complexity:  O(word_length + possible_length)
pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams
        .iter()
        .filter_map(|&s| is_anagram(word, s))
        .collect()
}
