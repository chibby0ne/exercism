pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let numbers: Vec<u64> = (2..=upper_bound).collect();
    let mut primes = vec![true; numbers.len()];
    for (i, prime) in (2..=upper_bound / 2).enumerate() {
        if !primes[i] {
            continue;
        }
        for non_prime in (prime + prime..=upper_bound).step_by(prime as usize) {
            if let Some(index) = numbers.iter().position(|&v| v == non_prime) {
                primes[index] = false;
            }
        }
    }
    numbers
        .iter()
        .enumerate()
        .filter(|(c, _)| primes[*c])
        .map(|(_, &v)| v)
        .collect()
}
