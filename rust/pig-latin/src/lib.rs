const PIG_LATIN_FRAGMENT: &'static str = "ay";

// fn starts_with_vowel(input: &str) -> bool {
//     input.starts_with('a') | input.starts_with('e') | input.starts_with('i') | input.starts_with('o') | input.starts_with('u')
// }

fn get_consonant_cluster(input: &str) -> String {
    input
        .chars()
        .take_while(|&v| v != 'a' && v != 'e' && v != 'i' && v != 'o' && v != 'u')
        .collect()
}

fn is_rule_1(input: &str) -> bool {
    get_consonant_cluster(input).is_empty()
        | input.starts_with("xr")
        | input.starts_with("yt")
}

fn is_rule_2(input: &str) -> bool {
    let consonant_cluster = get_consonant_cluster(input);
    let cluster_length = &consonant_cluster.len();
    !consonant_cluster.is_empty()
        && input.get(cluster_length - 1..cluster_length + 1) != Some("qu")
        && !consonant_cluster[1..].find('y').is_some()
}

fn is_rule_3(input: &str) -> bool {
    let consonant_cluster = get_consonant_cluster(input);
    let cluster_length = consonant_cluster.len();
    !consonant_cluster.is_empty()
        && input.get(cluster_length - 1..cluster_length + 1) == Some("qu")
        && !consonant_cluster[1..].find('y').is_some()
}

fn is_rule_4(input: &str) -> bool {
    let consonant_cluster = get_consonant_cluster(input);
    let cluster_length = consonant_cluster.len();
    !consonant_cluster.is_empty() && input.get(cluster_length..cluster_length + 1) == Some("y")
}

fn check_str_after_consonant_cluster(input: &str, s: &str) -> bool {
    let consonant_cluster = get_consonant_cluster(input);
    let cluster_length = consonant_cluster.len();
    input.get(cluster_length..cluster_length + 1) == Some(s)
}

pub fn translate(input: &str) -> String {
    let mut res = String::new();
    if input.is_empty() {
        return res;
    }
    for word in input.split_whitespace() {
        if is_rule_1(word) {
            println!("case 1");
            res.push_str(word);
            res.push_str(PIG_LATIN_FRAGMENT);
        } else if is_rule_2(word) {
            println!("case 2");
            let consonant_cluster = get_consonant_cluster(input);
            let start = consonant_cluster.len();
            // format!("{}{}{}", &input[start..], &input[..start], PIG_LATIN_FRAGMENT);
            res.push_str(&word[start..]);
            res.push_str(&word[..start]);
            res.push_str(PIG_LATIN_FRAGMENT);
        } else if is_rule_3(word) {
            println!("case 3");
            let consonant_cluster = get_consonant_cluster(input);
            let end = consonant_cluster.len() + 1;
            // format!("{}{}{}", &input[end..], &input[..end], PIG_LATIN_FRAGMENT)
            res.push_str(&word[end..]);
            res.push_str(&word[..end]);
            res.push_str(PIG_LATIN_FRAGMENT);
        } else {
            println!("case 4");
            let consonant_cluster = get_consonant_cluster(input);
            let start_y = consonant_cluster.find('y').unwrap();
            // format!("{}{}{}", &input[start..], &input[..start], PIG_LATIN_FRAGMENT);
            res.push_str(&word[start_y..]);
            res.push_str(&word[..start_y]);
            res.push_str(PIG_LATIN_FRAGMENT);
        }
        res.push(' ');
    }
    res.trim_end().to_owned()
}
