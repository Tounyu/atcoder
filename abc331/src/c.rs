#![allow(non_snake_case)]

use std::collections::BTreeMap;
use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize; n],
    }

    let mut memo = BTreeMap::new();

    for i in 0..n {
        let k = a[i];
        let mut v = memo.entry(k).or_insert(vec![]);
        v.push(i);
    }

    let mut ans = vec![0usize; n];
    let mut sum = 0;
    for (&k, &ref v) in memo.iter().rev() {

        for &i in v {
            ans[i] = sum;
        }

        sum += k * v.len();
    }

    for ans in ans {
        print!("{ans} ");
    }
}
