#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N:usize,
        A:[usize;N],
        B:[usize;N],
    }

    let mut memo = vec![];
    let mut sum = 0;
    let mut count = 0;
    for i in 0..N{
        if A[i] < B[i] {
            count+=1;
            sum += B[i] - A[i];
        } else {
            memo.push(A[i] - B[i]);
        }
    }

    memo.sort();
    let mut sum2 = 0;
    let mut count2 = 0;
    while !memo.is_empty() && sum > sum2 {
        sum2 += memo.pop().unwrap();
        count2 += 1;
    }

    if sum > sum2 {
        println!("-1");
    } else {
        println!("{}", count+count2);
    }
}
