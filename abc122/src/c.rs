#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        N:usize,
        Q:usize,
        S:Chars,
        LR:[(Usize1,Usize1);Q],
    }

    let mut sum = vec![0usize;N];
    for i in 1..N {
        if S[i] == 'C' && S[i-1] == 'A' {
            sum[i] = sum[i-1] + 1;
        } else {
            sum[i] = sum[i-1];
        }
    }
    for (l,r) in LR {
        let count = sum[r] - sum[l];
        println!("{count}");
    }
}
