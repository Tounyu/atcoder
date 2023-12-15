#![allow(non_snake_case)]

use std::collections::HashMap;
use proconio::input;

fn main() {
    input! {
        N:usize,
    }

    for i in 1..=N {
        let p = prime_factors(i);
        print!("{} ", p.iter().fold(0,|acc,(_, count)| acc + count) + 1);
    }
}

fn prime_factors(mut n: usize) -> HashMap<usize, usize> {
    let mut factors = HashMap::new();
    let mut i = 2;

    while i * i <= n {
        while n % i == 0 {
            *factors.entry(i).or_insert(0) += 1;
            n /= i;
        }
        i += 1;
    }

    if n > 1 {
        *factors.entry(n).or_insert(0) += 1;
    }

    factors
}
