#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
       mut S:Chars,
    }
    S.dedup();
    if S == ['A','B','C'] || S == ['A', 'B'] || S == ['B', 'C'] || S == ['A', 'C']|| S == ['A']  || S == ['B']|| S == ['C'] {
        println!("Yes");
    } else {
        println!("No");
    }
}
