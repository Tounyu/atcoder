#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        d:i64,
    }

    let mut ans = i64::MAX;

    let x = (d as f64).sqrt() as i64;
    for i in 1..=x {
        let c = calc(i, 0, d);
        if c >= 0 {
            ans = ans.min(c.abs());
        } else {
            let fl = (c.abs() as f64).sqrt() as i64;
            ans = ans.min(calc(i, fl, d).abs());
            ans = ans.min(calc(i, fl + 1, d).abs());
        }
    }
    println!("{}", ans);
}

fn calc(x:i64, y:i64, d:i64) -> i64 {
    x * x + y * y - d
}
