#![allow(non_snake_case)]

use ac_library::{Additive, Segtree};
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        N:usize,
        Q:usize,
        S:Chars,
        q:[(usize,usize,usize);Q],
    }

    let mut st = Segtree::<Additive<usize>>::new(N+1);
    for i in 0..N-1 {
        if S[i] != S[i+1] {
            st.set(i+1, 1);
        }
    }

    for (q, l,r) in q {
        if q == 1 {
            st.set(l-1, 1-st.get(l-1));
            st.set(r,1-st.get(r));
        } else {
            if st.prod(l..r) == r-l {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
