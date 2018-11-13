pub fn collatz(n: u64) -> Option<u64> {
    let mut num = n;
    let mut steps = 0;
    while num != 0 && num != 1 {
        match num % 2 {
            0 => num /= 2,
            _ => num = 3 * num + 1,
        };
        steps += 1;
    }
    match n {
        0 => None,
        1 => Some(0),
        _ => Some(steps),
    }
}
