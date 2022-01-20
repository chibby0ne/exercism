/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let iter = code.chars().filter_map(|x| x.to_digit(10));
    // If all chars are not digits or whitespaces the number is invalid according to the Luhn
    // formula
    // Likewise if has less than 2 digits (ignoring whitespaces) it is invalid
    if !code.chars().all(|x| x.is_digit(10) || x.is_whitespace()) || iter.clone().count() < 2 {
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
