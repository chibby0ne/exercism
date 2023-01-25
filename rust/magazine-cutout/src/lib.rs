// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut map: HashMap<&str, u64> = HashMap::new();
    for word in magazine {
        *map.entry(word).or_insert(0) += 1;
    }

    for word in note {
        match map.get_mut(word) {
            Some(mut v) => {
                if *v > 0 {
                    *v -= 1
                } else {
                    return false;
                }
            }
            None => return false,
        }
    }
    true
}
