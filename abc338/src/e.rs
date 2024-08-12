#![allow(non_snake_case)]

use std::collections::VecDeque;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        N:usize,
        mut AB:[(Usize1,Usize1);N],
    }
    let mut v = vec![(1,0);N*2];
    let l = 1;
    let r = 2;
    for (i, &(a,b)) in AB.iter().enumerate() {
        if a < b {
            v[a] = (l, i);
            v[b] = (r, i);
        } else {
            v[a] = (r, i);
            v[b] = (l, i);
        }
    }
    let mut st = VecDeque::new();
    for j in 0..N*2 {
        let (t, i) = v[j];
        if t == l {
            st.push_back(i);
        } else {
            let temp = st.pop_back().unwrap();
            if temp != i {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
