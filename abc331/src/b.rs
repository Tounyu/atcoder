#![allow(non_snake_case)]

use std::usize;
use proconio::input;

fn main() {
    input! {
        n:usize,
        s:usize,
        m:usize,
        l:usize,
    }

    let set = [(6, s), (8, m), (12, l)];
    let M = 10000000009usize;
    let mut dp = vec![M; n + 13];
    dp[0] = 0;

    for i in 1..=n + 12 {
        for (num, cost) in set {
            if i >= num {
                dp[i] = dp[i].min(dp[i - num] + cost);
            }
        }
    }

    let mut ans = M;
    for i in n..n+12 {
        ans = ans.min(dp[i]);
    }

    println!("{}", ans);
}
