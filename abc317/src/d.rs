#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        n:usize,
        set:[(i64,i64,i64);n],
    }
    let M = 1_000000000_000i64;
    let zsum = set.iter().fold(0, |acc, &s| acc + s.2);
    let mut dp = vec![M; (zsum + 1) as usize];
    dp[0] = 0;
    for i in 0..n {
        let (x, y, z) = set[i];
        let w = 0.max((x + y) / 2 + 1 - x);
        for j in (z as usize..=zsum as usize).rev() {
            dp[j] = dp[j].min(dp[j - z as usize] + w);
        }
    }
    let mut ans = M;
    for j in zsum / 2 + 1..=zsum {
        ans = ans.min(dp[j as usize]);
    }
    println!("{}", ans);
}
