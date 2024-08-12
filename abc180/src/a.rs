#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N:i32,
        A:i32,
        B:i32,
    }

    println!("{}", N - A + B);
}
