#![feature(exclusive_range_pattern)]

const UP_TO_NINETEEN: [&'static str; 20] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const DECENS: [&'static str; 9] = [
    "twenty",   // 0
    "thirty",   // 1
    "forty",    // 2
    "fifty",    // 3
    "sixty",    // 4
    "seventy",  // 5
    "eighty",   // 6
    "ninety",   // 7
    "hundred",  // 8
];

const THOUSANDS: [&'static str; 6] = [
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
];

fn handle_thousands(n: u64) -> String {
    let (mut digits, mut residue) = (n / 1_000, n % 1_000);
    let mut exp = 1;
    while dbg!(digits >= 1_000) {
        exp += 1;
        digits = n / u64::pow(1_000, exp);
        residue = n % u64::pow(1_000, exp);
    }
    if residue != 0 {
        format!("{} {} {}", encode(digits), THOUSANDS[exp as usize - 1], encode(residue))
    } else {
        format!("{} {}", encode(digits), THOUSANDS[exp as usize - 1])
    }
}

pub fn encode(n: u64) -> String {
    match n {
        0..20 => {
            format!("{}", UP_TO_NINETEEN[n as usize])
        },
        20..100 => {
            let (decen, residue) = (n / 10, n % 10);
            if residue != 0 {
                format!("{}-{}", DECENS[decen as usize - 2], encode(residue))
            } else {
                format!("{}", DECENS[decen as usize - 2])
            }
        },
        100..1000 => {
            let (hundred, residue) = (n / 100, n % 100);
            if residue != 0 {
                format!("{} {} {}", encode(hundred), DECENS[8], encode(residue))
            } else {
                format!("{} {}", encode(hundred), DECENS[8])
            }
        },
        _ => {
            handle_thousands(n)
        }
    }
}
