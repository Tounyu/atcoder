use ac_library::ModInt998244353;
use proconio::input;

// ModInt DP 確率
fn main() {
    input! {
        n:usize,
        x:usize,
        t:[usize;n],
    }
    let mut ans = ModInt998244353::new(0);
    // tのときの確率をdpしていく
    let mut dp = vec![ModInt998244353::new(0); x + 1];
    let r = ModInt998244353::new(1) / ModInt998244353::new(n);
    dp[0] = ModInt998244353::new(1);
    if t[0] > x {
        ans += dp[0] / ModInt998244353::new(n);
    }
    for i in 1..=x {
        for j in 0..n {
            if t[j] <= i {
                dp[i] = dp[i] + dp[i - t[j]];
            }
        }
        dp[i] *= r;
        if i + t[0] > x {
            ans += dp[i] * r;
        }
    }

    println!("{ans}");
}
