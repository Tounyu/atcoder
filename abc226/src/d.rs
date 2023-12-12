#![allow(non_snake_case)]

use std::collections::HashSet;
use proconio::input;

fn main() {
    input! {
        N:usize,
        xy:[(i64,i64);N],
    }

    let mut set = HashSet::new();

    for i in 0..N{
        for j in 0..N{
            if i == j {
                continue;
            }
            let a = xy[i].0 - xy[j].0;
            let b = xy[i].1 - xy[j].1;
            let g = gcd(a, b);
            set.insert((a/g,b/g));
        }
    }

    println!("{}", set.len() * 2);
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
