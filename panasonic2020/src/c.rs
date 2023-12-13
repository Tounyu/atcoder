#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        a:i64,
        b:i64,
        c:i64,
    }

    let d = c-a-b;
    if d > 0 && d * d > 4 * a * b {
        println!("Yes");
    } else {
        println!("No");
    }
}
