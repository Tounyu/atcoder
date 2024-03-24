#![allow(non_snake_case)]

use std::collections::HashSet;
use proconio::input;

fn main() {
    input! {
        N:usize,
        K:usize,
        A:[usize;N],
    }

    let mut S = K * (1 + K) / 2;
    let mut set = HashSet::new();

    for a in A {
        if a <= K {
            set.insert(a);
        }
    }

    for a in set {
        S -= a;
    }

    println!("{S}");
}
