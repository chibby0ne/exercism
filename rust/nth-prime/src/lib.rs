fn check_if_prime(vec: &Vec<u32>, value : u32) -> bool {
    for prime in vec.iter() {
        if value % prime == 0 {
            return false;
        }
    }
    return true;
}

pub fn nth(n: u32) -> Option<u32> {
    let mut primes_found : Vec<u32> = Vec::new();
    let mut count = 0;
    let mut num = 2;
    while count < n && num < std::u32::MAX {
        if check_if_prime(&primes_found, num) {
            count += 1;
            primes_found.push(num);
            if count == n {
                return Some(num);
            }
        }
        num += 1;
    }
    return None;
}
