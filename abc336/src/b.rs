#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        n:usize,
    }

    let mut s = format!("{:b}", n);
    let mut count = 0;
    while s.len() > 0 {
        let c = s.pop().unwrap();
        if c == '0' {
            count += 1;
        } else {
            break;
        }
    }
    println!("{count}");
}
