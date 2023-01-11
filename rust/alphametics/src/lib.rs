use std::collections::{HashSet, HashMap};


fn insert_char_if_not_seen(s: &str, hashmap: &mut HashSet<char>) {
    for c in s.chars() {
        if !hashmap.contains(&c) {
            hashmap.insert(c);
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

    input_as_numbers.iter().sum::<u32>() == result_as_number
}

struct Permutation {
    letters: Vec<char>,
    count: usize,
    max: usize,
    current_map: HashMap<char, u8>,
    current_values: Vec<u8>,
}

impl Permutation {

    fn new(s: &HashSet<char>) -> Self {
        fn combinations(num: usize) -> usize {
            let start = 10 - num + 1;
            let mut result = 1;
            for v in start..=10 {
                result *= v;
            }
            result
        }

        let mut m: HashMap<char, u8> = HashMap::new();

        for k in s {
            m.insert(*k, 0);
        }

        Self {
            letters: s.iter().copied().collect(),
            count: 0,
            max: combinations(s.len()),
            current_map: m,
            current_values: vec![0; s.len()],
        }
    }

    fn find_next_digit(self, index: u8, set: &HashSet<u8>) -> (bool, u8) {

    }


    fn permute(&mut self) -> HashMap<char, u8> {
        let mut s: HashSet<u8> = self.current_values.iter().copied().collect();

        // let mut stack: Vec<u8> = Vec::new();
        // let mut i = 0;
        // while i < self.current_values.len() {
        //     let (rollover, next_digit) = self.find_next_digit(i as u8, &s);
        //     if rollover == false {
        //         self.current_values[i] = next_digit;
        //         break;
        //     }
        //     stack.push(next_digit);
        //     i += 1;
        // }
        self.letters.iter().copied().zip(self.current_values.iter().copied()).collect()
    }
}


impl Iterator for Permutation {
    type Item = HashMap<char, u8>;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count == self.max {
            None
        } else {
            Some(self.permute())
        }

    }
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {

    // The HashMap must be mutable
    let mut set: HashSet<char> = HashSet::new();

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
    insert_char_if_not_seen(result, &mut set);



    for s in &input {
        insert_char_if_not_seen(s, &mut set);
    }

    // We iterate over each kind of 
    let perm = Permutation::new(&set);
    for hashmap in perm {
        if convert_to_numbers_and_check_result(&input, &result, &hashmap) {
            return Some(hashmap);
        }
    }
    None
}
