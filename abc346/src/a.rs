#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N:usize,
        A:[usize;N]
    }

    for i in 0..N-1 {
        print!("{} ", A[i] * A[i+1]);
    }
    println!();
}
