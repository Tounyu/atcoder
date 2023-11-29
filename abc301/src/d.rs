#![allow(non_snake_case)]

use proconio::input;
use proconio::marker::Chars;

// 2進数
fn main() {
    input! {
        S:Chars,
        n:u64,
    }

    let mut s = 0;
    for i in (0..S.len()).rev() {
        if S[i] == '1' {
            s += 1 << S.len() - 1 - i;
        }
    }
    if s > n {
        println!("-1");
        return;
    }
    for i in 0..S.len() {
        let x = 1 << S.len() - 1 - i;
        if S[i] == '?' && (s | x) <= n {
            s |= x;
        }
    }
    println!("{s}");
}
