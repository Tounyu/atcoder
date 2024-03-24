#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        A:usize,
        B:usize,
    }

    for i in 0..10 {
        if i != A + B {
            println!("{i}");
            return;
        }
    }
}
