#![allow(non_snake_case)]

use std::collections::HashSet;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        S:Chars,
    }
    let mut set = HashSet::new();
    for i in 0..S.len() {
        for j in i..S.len() {
            let str:String = S[i..=j].into_iter().collect();
            set.insert(str);
        }
    }
    println!("{}", set.len());
}
