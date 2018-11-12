pub fn raindrops(n: usize) -> String {
    let mut v = Vec::new();
    if n % 3 == 0 {
        v.push("Pling");
    } if n % 5 == 0 {
        v.push("Plang");
    } if n % 7 == 0 {
        v.push("Plong");
    }
    if v.len() == 0 {
        return n.to_string();
    }
    return v.join("");
}
