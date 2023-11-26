#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        n:usize,
        xl:[(i64,i64);n],
    }
    let mut set = vec![];
    for (x,l) in xl {
        set.push((x-l, x+l));
    }
    set.sort_by(|a,b| a.1.cmp(&b.1));
    let mut count = 1;
    let mut now = set[0].1;
    for i in 1..n {
        let (l,r) = set[i];
        if now <= l {
            count += 1;
            now = r;
        }
    }
    println!("{}", count);
}
