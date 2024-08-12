#![allow(non_snake_case)]

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        N:usize,
        S:Chars,
        C:[usize;N],
    }

    let mut l0 = vec![0usize; N+1];
    let mut l1 = vec![0usize; N+1];
    let mut r0 = vec![0usize; N+1];
    let mut r1 = vec![0usize; N+1];

    for i in 0..N {
        l0[i + 1] = l0[i];
        l1[i + 1] = l1[i];
        if i % 2 == 0 {
            if S[i] == '0' {
                l1[i+1] += C[i];
            } else {
                l0[i+1] += C[i];
            }
        } else {
            if S[i] == '0' {
                l0[i+1] += C[i];
            } else {
                l1[i+1] += C[i];
            }
        }
    }

    for i in (0..N).rev() {
        r0[i] = r0[i + 1];
        r1[i] = r1[i + 1];
        if i % 2 == 0 {
            if S[i] == '0' {
                r0[i] += C[i];
            } else {
                r1[i] += C[i];
            }
        } else {
            if S[i] == '0' {
                r1[i] += C[i];
            } else {
                r0[i] += C[i];
            }
        }
    }

    let mut ans = usize::MAX;

    for i in 1..N {
        ans = ans.min(l0[i] + r0[i]);
    }

    for i in 1..N {
        ans = ans.min(l1[i] + r1[i]);
    }
    println!("{ans}");
}
