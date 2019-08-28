fn is_equals_to_x(c: char) -> bool {
    c == 'X'
}

fn add_ten_if_x(c: char) -> u32 {
    if is_equals_to_x(c) {
        10
    } else {
        0
    }
}

/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    if isbn.is_empty() {
        return false;
    }
    let nums = isbn.chars().filter_map(|c| c.to_digit(10));
    let last = isbn.chars().last().unwrap();
    let count = nums.clone().count() + if is_equals_to_x(last) { 1 } else { 0 };
    if count != 10 {
        return false;
    }
    let mut sum: u32 = nums.enumerate().map(|(i, v)| v * (10 - i as u32)).sum();
    sum += add_ten_if_x(last);
    if sum % 11 == 0 {
        true
    } else {
        false
    }
}
