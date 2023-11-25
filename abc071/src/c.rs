#![allow(non_snake_case)]

use std::collections::BTreeMap;
use proconio::input;

// ordered map
fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut set: BTreeMap<usize,usize> = BTreeMap::new();
    for a in a {
        if set.contains_key(&a) {
            set.insert(a, set.get(&a).unwrap() + 1);
        } else {
            set.insert(a, 1);
        }
    }

    let mut ans = 0;

    // 長方形
    if let Some((x, _)) = set.iter().rfind(|&(k,v)| *v >= 2) {
        if let Some((y, _)) = set.iter().rfind(|&(k,v)| k != x && *v >= 2) {
            ans = ans.max(x * y);
        }
    }
    // 正方形
    if let Some((x, _)) = set.iter().rfind(|&(k,v)| *v >= 4) {
        ans = ans.max(x * x);
    }

    println!("{}", ans);
}
