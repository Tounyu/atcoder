#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        A:usize,
        B:usize,
    }

    let ff = 2 * A + 100 - B;
    println!("{}", ff);
}
