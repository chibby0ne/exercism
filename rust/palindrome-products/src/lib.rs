use std::iter;
/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct Palindrome(u64);

impl Palindrome {

    fn is_palindrome(value: u64) -> bool {
        let val_str = value.to_string();
        let f = val_str.chars();
        let r = val_str.chars().rev();
        f.zip(r).all(|(ff, rr)| ff == rr)
    }

    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        if Palindrome::is_palindrome(value) {
            Some(Palindrome(value))
        } else {
            None
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let products: Vec<u64> = (min..=max).flat_map(|v| iter::repeat(v).zip(min..=max).map(|(a, b)| a * b).collect::<Vec<u64>>()).collect();
    let mut palindromes: Vec<Palindrome> = products.iter().filter_map(|&v| Palindrome::new(v)).collect();
    palindromes.sort_unstable();
    match (palindromes.first(), palindromes.last()) {
        (Some(&min), Some(&max)) => Some((min, max)),
        _ => None,
    }

}
