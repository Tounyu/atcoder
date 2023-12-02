#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;

// 2次元累積和　二次元累積和
fn main() {
    input! {
        N:usize,
        Q:usize,
        P:[Chars; N],
        q:[(usize,usize,usize,usize); Q],
    }

    let mut cum_sum: Vec<Vec<usize>> = vec![vec![0; N + 1]; N + 1];

    for i in 1..=N {
        for j in 1..=N {
            cum_sum[i][j] = cum_sum[i - 1][j] + cum_sum[i][j - 1] - cum_sum[i - 1][j - 1];
            if P[i - 1][j - 1] == 'B' {
                cum_sum[i][j] += 1;
            }
        }
    }

    for (A,B,C,D) in q {
        let sum = calc(&cum_sum, C+1, D+1, N)
            + calc(&cum_sum,A, B, N)
            - calc(&cum_sum,A, D+1, N)
            - calc(&cum_sum, C+1, B, N);
        println!("{sum}");
    }
}

fn calc(cum_sum:&Vec<Vec<usize>>, a:usize,b:usize, N:usize) -> usize {
    if a <= N && b <= N {
        return cum_sum[a][b];
    }
    let mut ret = 0;
    let aa = a % N;
    let bb = b % N;
    ret += calc(cum_sum, N,N, N) * (a/N) * (b/N);
    ret += calc(cum_sum, aa, N, N) * (b/N);
    ret +=  calc(cum_sum,  N,bb, N) * (a/N);
    ret += calc(cum_sum, aa, bb, N);
    return ret;
}
