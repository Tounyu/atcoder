#![allow(non_snake_case)]

use std::collections::VecDeque;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        S:Chars,
    }
    let mut s = VecDeque::new();
    for i in (0..S.len()).rev() {
        if S[i] == '.' {
            break;
        }
        s.push_front(S[i]);
    }
    for i in 0..s.len() {
        print!("{}", s[i]);
    }
    println!();
}
