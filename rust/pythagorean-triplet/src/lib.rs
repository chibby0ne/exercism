pub fn find() -> Option<u32> {
    const SUM : u32 = 1000;
    for a in 1..SUM / 2 {
        for b in 1..SUM / 3 {
            let c : u32 = SUM - a - b;
            if a * a + b * b == c * c {
                return Some(a * b * c);
            }
        }
    }
    return None
}
