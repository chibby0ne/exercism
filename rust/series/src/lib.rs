pub fn series(digits: &str, len: usize) -> Vec<String> {
    let num_of_digits = digits.len();
    let mut temp: Vec<String> = Vec::new();
    if num_of_digits < len {
        return temp;
    } else if len == 0 {
        return vec!["".to_string(); num_of_digits + 1];
    }
    for i in 0..=num_of_digits - len {
        temp.push(digits.get(i..i + len).unwrap().to_string());
    }
    return temp;
}
