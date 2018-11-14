pub fn factors(n: u64) -> Vec<u64> {
    let mut output: Vec<u64> = Vec::new();
    let mut val = n;
    for i in 2..=val {
        while val % i == 0 {
            val /= i;
            output.push(i);
        }
        if val < 2 {
            break;
        }
    }
    output
}
