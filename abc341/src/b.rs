#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N:usize,
        mut A:[i64;N],
        st:[(i64,i64);N-1],
    }

    for (i,&(s,t)) in st.iter().enumerate() {
        let c = A[i] / s;
        A[i+1] += t * c;
    }

    println!("{}",A[N-1]);
}
