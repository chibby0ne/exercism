use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    let limit = num / 2;
    let sum: u64 = (1..=limit).filter(|val| num % val == 0).sum();
    match sum.cmp(&num) {
        Ordering::Less => Some(Classification::Deficient),
        Ordering::Equal => Some(Classification::Perfect),
        Ordering::Greater => Some(Classification::Abundant),
    }
}
