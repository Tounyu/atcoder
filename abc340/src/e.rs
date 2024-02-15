#![allow(non_snake_case)]

use ac_library::{LazySegtree, MapMonoid, Monoid};
use proconio::input;

// lazySegtree 区間更新

struct M;
impl Monoid for M {
    type S = isize;

    fn identity() -> Self::S {
        0
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        a+b
    }
}

struct F;
impl MapMonoid for  F {
    type M = M;
    type F = isize;

    fn identity_map() -> Self::F {
        0
    }

    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        f+x
    }

    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        f+g
    }
}

fn main() {
    input! {
        N:usize,
        m:usize,
        A:[isize; N],
        B:[usize; m],
    }

    let mut st = LazySegtree::<F>::new(N);

    for i in 0..N {
        st.apply(i, A[i]);
    }

    for i in B {
        let b = st.get(i);
        st.apply(i, -b);

        // 全体に加算
        st.apply_range(0..N, b / N as isize);

        // 余り
        let temp = b % N as isize;
        st.apply_range(i + 1..N.min(i + 1 + temp as usize), 1);
        if N < i + 1 + temp as usize {
            let temp2 = i + 1 + temp as usize - N;
            st.apply_range(0..temp2, 1);
        }
    }

    for i in 0..N {
        print!("{} ", st.get(i));
    }
    println!();
}
