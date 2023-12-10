#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        K:usize,
        G:usize,
        M:usize,
    }

    let mut g = 0;
    let mut m = 0;

    for _i in 0..K {
        if g == G {
            g = 0;
        } else if m == 0 {
            m = M;
        } else {
            while m != 0 && g != G {
                m -=1;
                g +=1;
            }
        }
    }

    println!("{g} {m}");
}
