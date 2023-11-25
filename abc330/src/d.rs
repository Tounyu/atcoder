#![allow(non_snake_case)]

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n:usize,
        s:[Chars;n],
    }

    let mut count = 0;
    let mut h = vec![0usize;n];
    let mut w = vec![0usize;n];
    for i in 0..n {
        for j in 0..n{
           if s[i][j] == 'o' {
               h[i] +=1;
               w[j] +=1;
           }
        }
    }
    for i in 0..n {
        for j in 0..n{
            if s[i][j] == 'o' {
                count += (h[i] - 1) * (w[j] - 1);
            }
        }
    }
    println!("{}", count);
}
