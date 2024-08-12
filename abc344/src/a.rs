#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        S:Chars,
    }

    let mut count = 0;
    for c in S {
        if c == '|' {
            count+=1;
        } else if count != 1 {
            print!("{c}");
        }
    }
}
