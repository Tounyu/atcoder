#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        a:f64,
        b:f64,
    }

    let x = (a / b / 2f64).powf(2f64/3f64);
    let mut ans = a;
    for i in [-2,-1,0,1,2] {
        let xx = x as i64 + i;
        ans = ans.min(calc(a,b,xx));
    }
    println!("{ans:.9}");
}

fn calc(a:f64,b:f64,n:i64) -> f64 {
    return a / (n as f64 + 1f64).sqrt() + (b * n as f64);
}
