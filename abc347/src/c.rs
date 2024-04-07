#![allow(non_snake_case)]

use std::collections::HashSet;
use proconio::input;

fn main() {
    input! {
        N:usize,
        A:usize,
        B:usize,
        D:[usize;N],
    }
    let mut set = HashSet::new();
    let S = A+B;
    for d in D {
        let dd = d % S;
        set.insert(dd);
    }

    let mut v: Vec<usize> = Vec::from_iter(set);
    v.sort();

    if v.len() == 1 {
        println!("Yes");
        return;
    }

    if v[v.len()-1] - v[0] < A {
        println!("Yes");
        return;
    }

    for i in 0..v.len()-1 {
        if (v[i + 1] - v[i]) > B {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
