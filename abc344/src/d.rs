#![allow(non_snake_case)]

use std::cmp::min;
use proconio::input;

fn main() {
    input! {
        T:String,
        N:usize,
    }

    let tl = T.len();

    let mut dp = vec![vec![10_i32.pow(9);  105]; 105];
    dp[0][0] = 0;
    for i in 0..N {
        for j in 0..105 {
            dp[i+1][j] = dp[i][j];
        }
        input! {
            A:usize,
            SS:[String; A],
        }
        for s in SS {
            let sl = s.len();
            if tl < sl {
                continue;
            }
            for j in 0..=tl-sl {
                let check = &T[j..j+sl];
                if s == check {
                    // println!("{s}, {check}");
                    dp[i+1][j+sl] = min(dp[i+1][j+sl],dp[i][j] + 1);
                }
            }
        }
    }

    if dp[N][tl] > 5_i32.pow(8) {
        println!("-1");
    } else {
        println!("{}", dp[N][tl]);
    }
}
