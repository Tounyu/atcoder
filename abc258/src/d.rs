#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N:usize,
        X:usize,
        AB:[(usize,usize);N],
    }

    let mut ans = usize::MAX;

    let mut memo = vec![0usize; N];
    memo[0] = AB[0].0 + AB[0].1;
    for i in 1..N {
        memo[i] = memo[i - 1] + AB[i].0 + AB[i].1;
    }

    for i in 0..N {
        let sum = memo[i] + AB[i].1 * (X - i - 1);
        ans = ans.min(sum);
    }

    println!("{ans}");
}
