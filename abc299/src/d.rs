#![allow(non_snake_case)]

use proconio::input_interactive;

fn main() {
    input_interactive! {
        N:usize,
    }

    let mut l = 1usize;
    let mut r = N;
    let mut mid : usize;
    for _i in 0..20 {
        if l + 1 == r {
            println!("! {}", l);
            return;
        }
        mid = (l + r) / 2;
        println!("? {mid}");
        input_interactive! {
            ans:i32,
        }
        if ans == 0 {
            l = mid;
        } else {
            r = mid;
        }
    }
}
