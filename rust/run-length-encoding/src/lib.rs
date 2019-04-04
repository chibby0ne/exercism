fn push_count_with_letter(letter: char, count: u64, string: &mut String) {
    if count > 1 {
        string.push_str(&(format!("{}{}", count, letter)));
    } else {
        string.push_str(&(format!("{}", letter)));
    }
}

pub fn encode(source: &str) -> String {
    let mut encoded = String::new();
    let previous = source.chars().nth(0);
    let mut count = 1;
    if let Some(mut prev_letter) = previous {
        for letter in source.chars().skip(1) {
            if letter == prev_letter {
                count += 1;
                continue;
            } else {
                push_count_with_letter(prev_letter, count, &mut encoded);
                prev_letter = letter;
                count = 1;
            }
        }
        let last_letter = source.chars().last().unwrap();
        push_count_with_letter(last_letter, count, &mut encoded);
    }
    encoded
}

pub fn decode(source: &str) -> String {
    let mut decoded = String::new();
    let mut prev_number = String::new();
    for letter in source.chars() {
        if letter.is_numeric() {
            prev_number = format!("{}{}", &prev_number, letter);
        } else {
            let _count: usize;
            match prev_number.parse() {
                Ok(count) => decoded.push_str(&(format!("{}", letter)).repeat(count)),
                _ => decoded.push(letter),
            }
            prev_number.clear();
        }
    }
    decoded
}
