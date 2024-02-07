#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        mut A:[isize;n],
    }

    let mut v = vec![0usize; n];
    let mut now = 0;
    for i in 0..n {
        if A[i] == -1 {
            now = i;
        } else {
            v[(A[i] - 1) as usize] = i;
        }
    }

    // println!("{:?}", v);
    let mut count = 0;
    while count < n {
        print!("{} ", now + 1);
        now = v[now];
        count +=1;
    }
}
