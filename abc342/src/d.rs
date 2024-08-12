#![allow(non_snake_case)]

use std::collections::BTreeMap;
use proconio::input;

fn main() {
    input! {
        N:usize,
        A:[usize;N],
    }
    let mut set = BTreeMap::new();
    let mut zeros = 0;
    for a in A {
        if a == 0 {
            zeros += 1;
            continue;
        }
        let mut d = 1;
        for i in 1..a {
            if a < i * i { break; }
            if a % (i * i) == 0 {
                d = i * i;
            }
        }
        *set.entry(a/d).or_insert(0) += 1;
    }

    let mut ans = 0;
    ans += zeros * (N - zeros);
    if zeros > 0 {
        ans += (zeros - 1) * zeros / 2;
    }
    for (k,v) in set {
        ans += v * (v - 1) / 2;
    }

    println!("{}", ans);
}
