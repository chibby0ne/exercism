use std::collections::HashMap;


fn insert_char_if_not_seen(s: &str, hashmap: &mut HashMap<char, u8>) {
    for c in s.chars() {
        if !hashmap.contains_key(&c) {
            hashmap.insert(c, 0);
        }
    }
}

fn get_number_representation(s: &str, hashmap: &HashMap<char, u8>) -> u32 {
    let mut chars = s.chars().rev();
    let mut number: u32 = *hashmap.get(&chars.next().unwrap()).unwrap() as u32;
    for c in chars {
        number *= 10;
        number += *hashmap.get(&c).unwrap() as u32;
    }
    number
}


fn convert_to_numbers_and_check_result(input: &Vec<&str>, result: &str, hashmap: &HashMap<char, u8>) -> bool {
    // Convert inputs to number
    let mut input_as_numbers: Vec<u32> = Vec::with_capacity(input.len());
    let iter = input.iter();
    for (i, s) in iter.enumerate() {
        input_as_numbers[i] = get_number_representation(s, &hashmap);
    }
    // Convert result to number
    let result_as_number = get_number_representation(&result, &hashmap);

    let addition: u32 = input_as_numbers.iter().sum();

    addition == result_as_number
}

struct Permutation {
    hashmap: HashMap<char, u8>,
    count: usize,
}

impl Permutation {
    fn new(h: &HashMap<char, u8>) -> Self {
        Self {
            hashmap: h.clone(),
            count: 0,
        }
    }
}

impl Iterator for Permutation {
    type Item = HashMap<char, u8>;

    fn next(&mut self) -> Option<Self::Item> {

    }
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {

    // The HashMap must be mutable
    let mut hashmap: HashMap<char, u8> = HashMap::new();

    // Get all the unique letters to create the hashmap
    let input_and_result: Vec<&str> = input.split("==").collect();

    // If any of the input or result is missing then there can be no solution and therefore we
    // return None
    // The input is split into a vector of &str since there could be > 1 inputs
    let input: Vec<&str> = match input_and_result.get(0) {
        Some(v) => v.split("+").map(|v| v.trim()).collect(),
        None => {
            return None;
        },
    };

    // The result is handled as a &str since it will only be 1 result
    let result = match input_and_result.last() {
        Some(v) => v.trim(),
        None => {
            return None;
        },
    };

    // There can be at most 10 entries to the hashmap since there can only be ten kinds of digits.
    insert_char_if_not_seen(result, &mut hashmap);



    for s in &input {
        insert_char_if_not_seen(s, &mut hashmap);
    }

    // We iterate over each kind of 
    for _ in 1..10 {
        if convert_to_numbers_and_check_result(&input, &result, &hashmap) {
            return Some(hashmap);
        }
    }
    None
}
