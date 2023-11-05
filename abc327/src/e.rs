use proconio::input;

// DP もらうDP 貰うDP
fn main() {
    input! {
        n:usize,
        p:[f64; n]
    }

    // dpで
    let mut dp = vec![vec![0f64; n]; n];
    dp[0][0] = p[0];
    for i in 1..n {
        for j in 0..=i {
            if j == 0 {
                dp[i][j] = dp[i - 1][j].max(p[i]);
            } else if j == i {
                dp[i][j] = dp[i - 1][j - 1] * 0.9 + p[i];
            } else {
                dp[i][j] = dp[i - 1][j].max(0.9 * dp[i - 1][j - 1] + p[i]);
            }
        }
    }

    let mut ans = f64::MIN;
    let mut d = 0.0;

    for k in 1..=n {
        d = d * 0.9 + 1.0;
        let res = dp[n - 1][k - 1] / d - 1200.0 / (k as f64).sqrt();
        ans = ans.max(res);
    }

    println!("{}", ans);
}
