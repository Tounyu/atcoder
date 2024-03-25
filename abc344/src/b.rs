#![allow(non_snake_case)]
use proconio::input;

fn main() {
    let mut s = vec![];
    for _ in 0..100{
        input! {
            A:usize,
        }
        s.push(A);
        if A == 0 {
            break;
        }
    }
    for &s in s.iter().rev() {
        println!("{s}");
    }
}
