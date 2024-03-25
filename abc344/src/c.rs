#![allow(non_snake_case)]

use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        N:usize,
        A:[i64; N],
        M:usize,
        B:[i64;M],
        L:usize,
        C:[i64;L],
        Q:usize,
        X:[i64;Q],
    }

    let mut temp = HashSet::new();
    for a in A {
        for b in &B {
            temp.insert(a+b);
        }
    }
    for x in X {
        let mut ans = false;
        for c in &C {
            if temp.contains(&(x - c)) {
                ans = true;
                break;
            }
        }
        if ans {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
