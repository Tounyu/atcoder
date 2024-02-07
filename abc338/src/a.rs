#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        S:Chars,
    }
    if !S[0].is_uppercase() {
        println!("No");
        return;
    }
    for i in 1..S.len() {
        if !S[i].is_lowercase() {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
