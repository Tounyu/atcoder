#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N:usize,
        M:usize,
        A:[i64; N-1],
       mut B:[usize; M],
    }

    let mut sabun = vec![];

    for i in 0..M {
        let b = B[i];
        let c = A[b];

        sabun.push((b,b, -c));
        let s = c / N;
    }
}
