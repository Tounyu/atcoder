#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        A:i64,
        M:i64,
        L:i64,
        R:i64,
    }

    let r = R - A;
    let l = L - A;
    let mut ans = 0;

    if l <= 0 && 0 <= r {
        ans += r / M;
        ans -= l / M;
        ans += 1;
    } else {
        if r > 0 {
            ans += r / M;
            ans -= l / M;
            ans = ans.abs();
            if l % M == 0{
                ans += 1;
            }
        } else {
            ans += l / M;
            ans -= r /M;
            ans = ans.abs();
            if r % M == 0{
                ans += 1;
            }
        }
    }
    println!("{}",ans);
}
