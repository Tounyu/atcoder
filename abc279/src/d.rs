#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        A:i64,
        B:i64,
    }

    let mut m = A as f64;
    for i in 0..=A/B {
        let ans = calc(i, A, B);
        m = m.min(ans);
    }

    println!("{m}");
}

fn calc(i:i64, A:i64, B:i64) -> f64 {
    return (A as f64 / ((1 + i) as f64).sqrt()) + ((i * B) as f64);
}