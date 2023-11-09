use proconio::input;

fn main() {
    input! {
        a:i64,
        b:i64,
        n:usize,
    }
    let mut ans = 0.max(calc(a, b, n as i64));
    if b-1 <= n as i64 {
        ans = ans.max(calc(a,b,b-1));
    }
    println!("{}", ans);
}

fn calc(a:i64, b:i64, x:i64) -> i64 {
    return (a * x) / b - a * (x / b);
}
