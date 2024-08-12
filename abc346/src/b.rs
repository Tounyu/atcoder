#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        W:usize,
        B:usize,
    }

    let mut S = String::new();
    for _ in 0..20 {
        S.push_str("wbwbwwbwbwbw");
    }
    let SS : Vec<char> = S.chars().collect();

    for s in 0..12 {
        let mut w = 0;
        let mut b = 0;

        for i in s..SS.len() {
            if SS[i] == 'w' {
                w += 1;
            } else {
                b += 1;
            }

            if w == W && b == B {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
