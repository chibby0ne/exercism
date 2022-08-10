/// time: [22.240 ns 22.821 ns 23.608 ns]
/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let (mut sum, mut numbers): (u32, u32) = (0, 0);
    for c in code.bytes().rev() {
        match c {
            b'0'..=b'9' => {
                let mut val = (c - b'0') as u32;
                if val != 0 && numbers % 2 == 1 {
                    val *= 2;
                    if val > 9 {
                        val -= 9;
                    }
                }
                sum += val;
                numbers += 1;
            }
            b' ' => {}
            _ => {
                return false;
            }
        }
    }
    numbers > 1 && sum % 10 == 0
}

/// time: [108.66 ns 109.53 ns 110.41 ns]
/// Check a Luhn checksum.
pub fn is_valid2(code: &str) -> bool {
    let iter = code.chars().filter_map(|x| x.to_digit(10));
    // If all chars are not digits or whitespaces the number is invalid according to the Luhn
    // formula
    // Likewise if has less than 2 digits (ignoring whitespaces) it is invalid
    if !code.chars().all(|x| x.is_ascii_digit() || x.is_whitespace()) || iter.clone().count() < 2 {
        false
    } else {
        // It is valid if the sum of the adjusted double of every second digit from the right, with all the other digits is a multiple of 10
        let res = iter
            .clone()
            .rev()
            .skip(1)
            .step_by(2)
            .map(|x| {
                let res = x * 2;
                if res > 9 {
                    res - 9
                } else {
                    res
                }
            })
            .chain(iter.clone().rev().step_by(2));
        res.sum::<u32>() % 10 == 0
    }
}

/// time:   [109.95 ns 111.70 ns 113.62 ns]
/// Check a Luhn checksum.
pub fn is_valid3(code: &str) -> bool {
    let (mut sum, mut numbers): (u32, u32) = (0, 0);
    for c in code.chars().rev() {
        match c.to_digit(10) {
            Some(mut val) => {
                if val != 0 && numbers % 2 == 1{
                    val *= 2;
                    if val > 9 {
                        val -= 9;
                    }
                }
                sum += val;
                numbers += 1;
            }
            None => {
                if c != ' ' {
                    return false;
                }
            }
        }
    }
    numbers > 1 && sum % 10 == 0
}
