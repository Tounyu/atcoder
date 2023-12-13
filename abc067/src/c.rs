#![allow(non_snake_case)]

use ac_library::{Additive, Segtree};
use proconio::input;

fn main() {
    input! {
        N:usize,
        A:[i64;N],
    }

    let mut segt = Segtree::<Additive<i64>>::new(N);
    for i in 0..N {
        segt.set(i, A[i]);
    }
    let mut ans = i64::MAX;
    for i in 1..N {
        let x = segt.prod(0..i);
        let y = segt.prod(i..N);
        ans = ans.min((x-y).abs());
    }
    println!("{}", ans);
}
