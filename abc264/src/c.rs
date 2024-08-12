#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        h1:usize,
        w1:usize,
        A:[[usize;w1];h1],
        h2:usize,
        w2:usize,
        B:[[usize;w2];h2],
    }

    for w in (0..w1).combinations(w2) {
        for h in (0..h1).combinations(h2) {
            let mut temp = vec![vec![0usize; w2]; h2];
            for (i, &ai) in h.iter().enumerate() {
                for (j, &aj) in w.iter().enumerate() {
                    temp[i][j] = A[ai][aj];
                }
            }
            if temp == B {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
