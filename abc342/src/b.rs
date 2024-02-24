#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N:usize,
        P:[usize;N],
        Q:usize,
        AB:[(usize,usize);Q],
    }

    for (a, b) in AB {
        for i in 0..N {
            if P[i] == a {
                println!("{}", a);
                break;
            }
            if P[i] == b {
                println!("{}", b);
                break;
            }
        }
    }
}
