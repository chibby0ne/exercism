extern crate rand;

use rand::prelude::*;

pub fn private_key(p: u64) -> u64 {
    let mut rng = thread_rng();
    rng.gen_range(2, p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_exponentiation(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_exponentiation(b_pub, a, p)
}

fn modular_exponentiation(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    match modulus {
        1 => 0,
        _ => {
            let mut result = 1;
            base %= modulus;
            while exponent > 0 {
                if exponent % 2 == 1 {
                    result = (result * base) % modulus;
                }
                exponent >>= 1;
                base = (base * base) % modulus;
            }
            result
        }
    }
}
