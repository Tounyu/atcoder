#![allow(non_snake_case)]

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        S:Chars,
    }
    let mut memo = vec![0; 28];
    for s in S {
        let i = s as u8 - 'a' as u8;
        memo[i as usize] += 1;
    }
    let mut count = memo[0];
    let mut ans = 0;
    for i in 0..memo.len() {
        if count < memo[i] {
            count = memo[i];
            ans = i;
        }
    }
    println!("{}", (ans as u8 + 'a' as u8) as char);
}
