#![allow(non_snake_case)]

use std::ffi::c_uint;
use proconio::input;

fn main() {
    input! {
        n:usize,
        s:usize,
        k:usize,
        pq:[(usize,usize);n],
    }

    let mut sum = 0;
    for (p,q) in pq {
        sum += p * q;
    }

    if sum < s {
        sum += k;
    }

    println!("{sum}");

}
