use std::fmt::Display;

pub struct Luhn {
    value: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let length = self.value.len();

        // Length <= 1 are invalid
        if length <= 1 {
            return false;
        }

        // Allocate a vector with as much capacity as there are chars
        let mut array_digits: Vec<u64> = Vec::with_capacity(length);

        // Starting from the right double every second number and if the result bigger than 9, substract
        // 9, then add all the digits including the ones not modified and if the sum is evenly
        // divisible by 10 it is Luhn valid
        for (i, c) in self.value.chars().rev().enumerate() {
            // Spaces are allowed in input but they should be stripped (or ignored)
            if c.is_whitespace() {
                continue;
            }
            // Any other non-digit char is disallowed
            if !c.is_ascii_digit() {
                return false;
            }
            // Convert the char to digit
            let mut num = c.to_digit(10).expect("Expected a number");
            // If it's every second we double it and if it's bigger we substract by 9
            if i % 2 == 1 {
                num *= 2;
                if num > 9 {
                    num -= 9;
                }
            }
            array_digits.push(num as u64);
        }
        array_digits.iter().sum::<u64>() % 10 == 0
    }
}

impl<T: Display> From<T> for Luhn {
    fn from(input: T) -> Self {
        // Convert to String and retain all the non-whitespace characters of the String
        // representation
        let mut s: String = input.to_string();
        s.retain(|c| !c.is_whitespace());
        Self { value: s }
    }
}

// /// Here is the example of how the From trait could be implemented
// /// for the &str type. Naturally, you can implement this trait
// /// by hand for the every other type presented in the test suite,
// /// but your solution will fail if a new type is presented.
// /// Perhaps there exists a better solution for this problem?
// impl<'a> From<&'a str> for Luhn {
//     fn from(input: &'a str) -> Self {
//         unimplemented!("From the given input '{}' create a new Luhn struct.", input);
//     }
// }
