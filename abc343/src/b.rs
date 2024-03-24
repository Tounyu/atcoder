#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N:usize,
        G:[[usize; N]; N],
    }

    for i in 0..N{
        for j in 0..N{
            if G[i][j] > 0 {
                print!("{} ", j+1);
            }
        }
        println!();
    }
}
