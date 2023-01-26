// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut map = magazine.iter().fold(HashMap::new(), |mut m, word| {
        *m.entry(word).or_insert(0) += 1;
        m
    });

    for word in note {
        let entry = map.entry(word).or_insert(0);
        if *entry == 0 {
            return false;
        }
        *entry -= 1;
    }
    true
}
