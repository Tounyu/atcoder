#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        n:usize,
        s:usize,
        ab:[(usize,usize);n],
    }

    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;

    // dp
    for i in 0..n {
        let (a,b) = ab[i];
        for j in 0..s {
            if dp[i][j] && j + a <= s {
                dp[i+1][j+a] = true;
            }
            if dp[i][j] && j + b <= s {
                dp[i+1][j+b] = true;
            }
        }
    }

    if dp[n][s] {
        println!("Yes");
        let mut ans = String::new();
        let mut now = s;
        for i in (0..n).rev() {
            let (a, b) = ab[i];
            if now >= a && dp[i][now - a] {
                now -= a;
                ans.push('H');
            } else {
                now -= b;
                ans.push('T');
            }
        }
        while !ans.is_empty() {
            print!("{}", ans.pop().unwrap());
        }
    } else {
        println!("No");
    }
}
