#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

const VALID_CHARS: [char; 6] = ['{', '}', '[', ']', '(', ')'];

fn is_valid_char(array: &[char], c: &char) -> bool {
    let mut is_valid = false;
    for x in array {
        if *x == *c {
            is_valid = true;
            break;
        }
    }
    is_valid
}

lazy_static! {
    static ref BRACKETS: HashMap<char, char> = {
        let mut map = HashMap::new();
        let mut i = 0;
        while i < VALID_CHARS.len() {
            map.insert(VALID_CHARS[i], VALID_CHARS[i + 1]);
            i += 2;
        }
        map
    };
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for ch in string.chars() {
        if is_valid_char(&VALID_CHARS, &ch) {
            match stack.last() {
                Some(bracket) => {
                    if Some(&ch) == BRACKETS.get(&bracket) {
                        stack.pop();
                    } else {
                        stack.push(ch);
                    }
                }
                _ => stack.push(ch),
            }
        }
    }
    stack.len() == 0
}
