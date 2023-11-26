#![allow(non_snake_case)]

use std::collections::VecDeque;
use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        mut a:[usize;n],
        mut bc:[(usize,usize);m],
    }
    a.sort();
    bc.sort_by(|a,b| b.1.cmp(&a.1));

    let mut aa = VecDeque::from(a.clone());
    for (b,c) in bc {
        for _i in 0..b {
            if aa[0] < c {
                aa.push_back(c);
                aa.pop_front();
            } else {
                break;
            }
        }
    }

    let mut sum = 0;
    for a in aa {
        sum += a;
    }
    println!("{sum}");
}
