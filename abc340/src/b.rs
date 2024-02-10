#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        Q:usize,
        q:[(usize,usize);Q],
    }

    let mut a = vec![];
    for (q, x) in q {
        if q == 1 {
            a.push(x);
        }
        if q == 2 {
            let len = a.len();
            println!("{}", a[len-x]);
        }
    }
}
