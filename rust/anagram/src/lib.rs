use std::collections::{HashMap, HashSet};

// Time complexity: O(word_length + possible_length)
// Space complexity: O(word_length + possible_length)
fn is_anagram(word: &str, possible: &str) -> bool {
    if word.len() != possible.len() {
        return false;
    }
    let mut map_word: HashMap<char, u32> = HashMap::new();
    let mut map_possible: HashMap<char, u32> = HashMap::new();
    for letter in possible.chars() {
        map_possible
            .entry(letter)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    for letter in word.chars() {
        map_word
            .entry(letter)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    for (letter, count) in map_word {
        match map_possible.get(&letter) {
            Some(&count_possible) => {
                if count_possible != count {
                    return false;
                }
            }
            None => {
                return false;
            }
        }
    }
    true
}

// Time complexity: O(possible_anagrams_length * O(word_length + MAX(possible_anagrams_length))
// Space complexity:  O(word_length + possible_length)
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result: HashSet<&str> = HashSet::new();
    let word_lowercase = word.to_lowercase();
    for possible in possible_anagrams {
        let possible_lowercase = possible.to_lowercase();
        if !possible_lowercase.eq(word_lowercase.as_str())
            && is_anagram(word_lowercase.as_str(), possible_lowercase.as_str())
        {
            result.insert(possible);
        }
    }
    result
}
