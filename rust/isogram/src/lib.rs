pub fn check(candidate: &str) -> bool {
    let mut it: Vec<char> = candidate
        .to_lowercase()
        .chars()
        .filter(|x| x.is_alphabetic())
        .collect();
    let len_before = it.len();
    it.sort_unstable();
    it.dedup();
    let len_after = it.len();
    len_before == len_after
}
