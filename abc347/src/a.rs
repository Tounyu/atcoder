#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N:usize,
        K:usize,
        A:[usize;N],
    }

    for a in A{
        if a % K == 0 {
            print!("{} ", a/ K);
        }
    }
}
