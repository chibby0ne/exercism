const WORDS: [&'static str; 27] = [
    "zero",     // 0
    "one",      // 1
    "two",      // 2
    "three",    // 3
    "four",     // 4
    "five",     // 5
    "six",      // 6
    "seven",    // 7
    "eight",    // 8
    "nine",     // 9
    "ten",      // 10
    "eleven",   // 11
    "twelve",   // 12
    "thir",     // 13
    "for",      // 14
    "fif",      // 15
    "teen",     // 16
    "twen",     // 17
    "ty",       // 18
    "hundred",  // 19
    "thousand", // 20
    "m",        // 21
    "b",        // 22
    "tr",       // 23
    "quadr",    // 24
    "quint",    // 25
    "illion",   // 26
];

fn handle_units(n: u64) -> String {
    format!("{}", WORDS[n as usize])
}

fn handle_teens(n: u64) -> String {
    if n >= 13 {
        let val = n % 10;
        let prefix = match val {
            4 => {
                format!("{}", WORDS[val as usize])
            }
            8 => {
                let word = WORDS[val as usize];
                let (eigh, _) = word.split_at(word.len() - 1);
                format!("{}", eigh)
            }
            _ => format!("{}", WORDS[10 + val as usize]),
        };
        format!("{}{}", prefix, WORDS[16])
    } else {
        handle_units(n)
    }
}

fn handle_decens(n: u64) -> String {
    let decen = n / 10;
    let res = n % 10;
    if decen >= 2 {
        let prefix = match decen {
            2 => format!("{}", WORDS[17 as usize]),
            3...6 => format!("{}", WORDS[10 + decen as usize]),
            8 => {
                let word = WORDS[decen as usize];
                let (eigh, _) = word.split_at(word.len() - 1);
                format!("{}", eigh)
            }
            _ => format!("{}", WORDS[decen as usize]),
        };
        let decens = format!("{}{}", prefix, WORDS[18]);
        if res != 0 {
            format!("{}-{}", decens, encode(res))
        } else {
            decens
        }
    } else {
        handle_teens(n)
    }
}

fn handle_hundreds(n: u64) -> String {
    let hun = n / 100;
    let res = n % 100;
    if hun >= 1 {
        let prefix = handle_units(hun);
        let hundreds = format!("{} {}", prefix, WORDS[19]);
        if res != 0 {
            format!("{} {}", hundreds, encode(res))
        } else {
            hundreds
        }
    } else {
        handle_decens(n)
    }
}

fn handle_thousands(n: u64) -> String {
    let tho = n / 1_000;
    let res = n % 1_000;
    if tho >= 1 {
        let prefix = handle_hundreds(tho);
        let thousands = format!("{} {}", prefix, WORDS[20]);
        if res != 0 {
            format!("{} {}", thousands, encode(res))
        } else {
            thousands
        }
    } else {
        handle_hundreds(n)
    }
}

fn handle_millions(n: u64) -> String {
    let mil = n / 1_000_000;
    let res = n % 1_000_000;
    if mil >= 1 {
        let prefix = handle_thousands(mil);
        let millions = format!("{} {}{}", prefix, WORDS[21], WORDS[26]);
        if res != 0 {
            format!("{} {}", millions, encode(res))
        } else {
            millions
        }
    } else {
        handle_thousands(n)
    }
}

fn handle_billions(n: u64) -> String {
    let bil = n / 1_000_000_000;
    let res = n % 1_000_000_000;
    if bil >= 1 {
        let prefix = handle_millions(bil);
        let billions = format!("{} {}{}", prefix, WORDS[22], WORDS[26]);
        if res != 0 {
            format!("{} {}", billions, encode(res))
        } else {
            billions
        }
    } else {
        handle_millions(n)
    }
}

fn handle_trillions(n: u64) -> String {
    let tri = n / 1_000_000_000_000;
    let res = n % 1_000_000_000_000;
    if tri >= 1 {
        let prefix = handle_billions(tri);
        let trillions = format!("{} {}{}", prefix, WORDS[23], WORDS[26]);
        if res != 0 {
            format!("{} {}", trillions, encode(res))
        } else {
            trillions
        }
    } else {
        handle_billions(n)
    }
}

fn handle_quadrillions(n: u64) -> String {
    let qua = n / 1_000_000_000_000_000;
    let res = n % 1_000_000_000_000_000;
    if qua >= 1 {
        let prefix = handle_trillions(qua);
        let quadrillions = format!("{} {}{}", prefix, WORDS[24], WORDS[26]);
        if res != 0 {
            format!("{} {}", quadrillions, encode(res))
        } else {
            quadrillions
        }
    } else {
        handle_trillions(n)
    }
}

fn handle_quintillions(n: u64) -> String {
    let quin = n / 1_000_000_000_000_000_000;
    let res = n % 1_000_000_000_000_000_000;
    if quin >= 1 {
        let prefix = handle_quadrillions(quin);
        let quintillions = format!("{} {}{}", prefix, WORDS[25], WORDS[26]);
        if res != 0 {
            format!("{} {}", quintillions, encode(res))
        } else {
            quintillions
        }
    } else {
        handle_quadrillions(n)
    }
}

pub fn encode(n: u64) -> String {
    match n {
        0..=12 => handle_units(n),
        13..=19 => handle_teens(n),
        20..=99 => handle_decens(n),
        100..=999 => handle_hundreds(n),
        1000..=999_999 => handle_thousands(n),
        1_000_000..=999_999_999 => handle_millions(n),
        1_000_000_000..=999_999_999_999 => handle_billions(n),
        1_000_000_000_000..=999_999_999_999_999 => handle_trillions(n),
        1_000_000_000_000_000..=999_999_999_999_999_999 => handle_quadrillions(n),
        _ => handle_quintillions(n),
    }
}
