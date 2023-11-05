use proconio::input;

fn main() {
    input! {
        n:usize,
        a1:[usize; n],
        a2:[usize; n],
    }

    let mut dp1 = vec![0usize; n];
    // a1を前から累積和
    dp1[0] = a1[0];
    for i in 1..n {
        dp1[i] = dp1[i-1] + a1[i];
    }
    // a2を後ろから累積和
    let mut dp2 = vec![0usize; n];
    dp2[n-1] = a2[n-1];
    for i in (0..n-1).rev() {
        dp2[i] = dp2[i+1] + a2[i];
    }

    let mut ans = 0usize;
    for i in 0..n {
        ans = ans.max(dp1[i] + dp2[i]);
    }
    println!("{}", ans);
}
