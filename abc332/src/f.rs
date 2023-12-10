#![allow(non_snake_case)]

use ac_library::{LazySegtree, ModInt998244353};
use itertools::min;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        N:usize,
        M:usize,
        A:[usize;N],
        Q:[(Usize1,Usize1,usize);M],
    }

    let mut seg = LazySegtree::new(N);

    for i in 0..N {
        seg.set(i, ModInt998244353::new(A[i]));
    }

    for (l,r,x) in Q {
        let n = ModInt998244353::new(r-l);
        let mintx = ModInt998244353::new(x);
        for i in l..=r {
            seg.apply_range(l..=r, |  a | a + n);
        }
    }

    for memo in memo {
        let mut ans = ModInt998244353::new(0);
        let mintn = ModInt998244353::new(memo.len());
        for mint in memo {
            ans += mint;
        }
        ans /= mintn;
        print!("{} ", ans);
    }
}
