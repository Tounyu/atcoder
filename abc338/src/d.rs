#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N:usize,
        M:usize,
        X:[usize;M],
    }

    let mut memo = vec![0i64; N];
    let mut now = X[0] - 1;
    for i in 1..M {
        println!("{:?}",memo);
        let next1 = X[i] - 1;
        let mut s = now;
        let mut e = next1;
        if next1 < now {
            s = next1;
            e = now;
        }

        // if s < e {
            if e - s < N / 2 {
                for j in s..e {
                    memo[j] += 1;
                }
            } else {
                for j in 0..N {
                    memo[j] += 1;
                }
                for j in s..e {
                    memo[j] -= 1;
                }
            }
        // } else {
        //     if s - next1 < N / 2 {
        //         for j in (s..next1).rev() {
        //             memo[j] += 1;
        //         }
        //     } else {
        //         for j in 0..N {
        //             memo[j] += 1;
        //         }
        //         for j in (s..next1).rev() {
        //             memo[j] -= 1;
        //         }
        //     }
        // }
        now = next1;
    }

    println!("{:?}", memo);
    memo.sort();
    let cut = memo[0];
    let mut sum = 0;
    for i in 1..N {
        sum += cut + memo[i];
    }
    println!("{sum}");
}
