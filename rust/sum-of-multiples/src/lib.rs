use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut set = HashSet::new();

    for factor in factors {
        let multiples: u32 = limit / factor;
        for multiple in 1..=multiples {
            // All multiples up to but NOT including that number (limit)
            if factor * multiple != limit {
                set.insert(factor * multiple);
            }
        }
    }
    set.iter().sum()
}
