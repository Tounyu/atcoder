#![allow(non_snake_case)]

use ac_library::{Max, Segtree};
use proconio::input;

fn main() {
    input! {
        N:usize,
        D:usize,
        A:[usize;N]
    }
    let size = 500010;
    let mut t = Segtree::<Max<usize>>::new(size);
    for a in A {
        let l = if a > D { a-D } else { 0 };
        let r = if a + D > size - 1 { size - 1 } else { a + D};
        let mx = t.prod(l..(r + 1));
        t.set(a, mx + 1);
    }
    let ans = t.all_prod();
    println!("{ans}");
}
