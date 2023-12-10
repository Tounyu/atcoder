#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        N:usize,
        M:usize,
        S:Chars
    }

    for i in 0..=N {
        if check(&S,M,i) {
            println!("{i}");
            return;
        }
    }
}

fn check(S:&Vec<char>, M:usize, L:usize) -> bool {
    let mut white = M;
    let mut logo = L;
    for (i, &c) in S.iter().enumerate() {
        if c == '0' {
            white = M;
            logo = L;
        }
        if c == '1' {
            if white > 0 {
                white -= 1;
            } else {
                if logo > 0 {
                    logo -= 1;
                } else {
                    return false;
                }
            }
        }
        if c == '2' {
            if logo > 0 {
                logo -= 1;
            } else {
                return false;
            }
        }
    }
    true
}