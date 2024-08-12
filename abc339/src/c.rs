#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N:usize,
        A:[i64;N],
    }

    let mut now = 0;
    for a in A {
        now += a;
        if now < 0 {
            now = 0;
        }
    }

    println!("{now}");
}
