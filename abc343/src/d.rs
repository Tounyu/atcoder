#![allow(non_snake_case)]

use std::collections::HashMap;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        N:usize,
        T:usize,
        AB:[(Usize1,usize);T],
    }

    let mut map = HashMap::new();
    let mut c = vec![0usize; N];
    map.insert(0, N);

    for (a, b) in AB {
        if let Some(&n) = map.get(&c[a]) {
            map.insert(c[a], n - 1);
            if n == 1 {
                map.remove(&c[a]);
            }
        }
        c[a] += b;
        *map.entry(c[a]).or_insert(0) += 1usize;

        println!("{}", map.len());
    }
}
