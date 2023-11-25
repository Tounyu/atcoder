#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        n:usize,
        l:usize,
        a:[usize;n],
    }
    let ans = a.iter().filter(|&x| l <= *x).count();
    println!("{}",ans);
}
