#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        B:usize,
        G:usize,
    }

    if B > G {
        println!("Bat");
    } else {
        println!("Glove");
    }

}
