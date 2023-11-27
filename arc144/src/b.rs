#![allow(non_snake_case)]

use std::collections::{BTreeMap, BTreeSet};
use proconio::input;

fn main() {
    input! {
        N:usize,
        a:i64,
        b:i64,
        A:[i64;N],
    }
    let mut set = BTreeSet::new();
    for x in A {
        set.insert(x);
    }
    let mut ans = set.first().unwrap();

}
