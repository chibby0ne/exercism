pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64");
    }
    2_u64.pow(s - 1) as u64
}

pub fn total() -> u64 {
    let mut squares: Vec<u64> = Vec::new();
    for i in 1..=64 {
        squares.push(square(i));
    }
    squares.iter().fold(0, |acc, &x| acc + x)
}
