#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N:usize,
        A:[i64;N],
    }
    let mut sum = vec![];
    sum.push((A[0], A[0].max(0)));
    for i in 1..N {
        let pos = sum[i-1].0 + A[i];
        let posM = sum[i-1].1.max(pos);
        sum.push((pos,posM));
    }
    let mut ans = 0;
    let mut pos = 0;
    for i in 0..N {
        ans = ans.max(pos + sum[i].1);
        pos += sum[i].0;
    }
    println!("{ans}");
}
