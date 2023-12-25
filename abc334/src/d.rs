#![allow(non_snake_case)]
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        N:usize,
        Q:usize,
        mut R:[usize;N],
        q:[usize;Q],
    }

    R.sort();
    let mut sum = vec![0; N];
    sum[0] = R[0];
    for i in 1..N {
        sum[i] = sum[i-1] + R[i];
    }

    for x in q {
        let ans = sum.lower_bound(&(&x + 1));
        println!("{ans}");
    }
}
