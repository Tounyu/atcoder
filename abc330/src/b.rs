#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        n:usize,
        l:usize,
        r:usize,
        a:[usize;n],
    }

    for a in a {
        let x = if a < l { l } else if a > r { r } else { a };
        print!{"{x} "};
    }
    println!();
}
