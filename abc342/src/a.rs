#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        S:Chars,
    }

    let s = S[0];
    let mut c = 1;
    let mut d = 0;
    for i in 1..S.len() {
        if s == S[i] {
            c += 1;
        } else {
            d = i;
        }
    }
    if c == 1 {
        println!("1");
    } else {
        println!("{}", d + 1);
    }
}
