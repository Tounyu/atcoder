#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        n:usize,
        mut ab:[(usize,usize);n],
    }
    ab.sort_by(|&a, &b| a.1.cmp(&b.1));

    let mut time = 0usize;
    for (a,b) in ab {
        time += a;
        if time <= b {
            continue;
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
