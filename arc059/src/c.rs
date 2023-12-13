#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N:usize,
        A:[i64;N],
    }
    let sum = A.iter().fold(0, |acc,&x| acc+x);
    let ave = sum / N as i64;
    let mut cost1 = 0;
    let mut cost2 = 0;
    for a in A {
        let d1 = a - ave;
        cost1 += d1 * d1;
        let d2 = a - ave - 1;
        cost2 += d2 * d2;
    }
    println!("{}", cost1.min(cost2));
}
