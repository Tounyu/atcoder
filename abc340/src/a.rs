#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        A:usize,
        B:usize,
        D:usize,
    }
    let mut i = A;
    while i <= B {
        print!("{i} ");
        i += D;
    }
    println!();
}
