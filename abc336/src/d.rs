#![allow(non_snake_case)]

use itertools::min;
use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize; n],
    }

    let mut l = vec![0usize; n+2];
    let mut r = vec![0usize; n+2];

    for i in 1..=n{
        l[i] = a[i-1].min(l[i-1] + 1);
    }
    for i in (1..=n).rev(){
        r[i] = a[i-1].min(r[i+1] + 1);
    }


    let mut ans = 1;
    for i in 1..=n {
        ans = ans.max(l[i].min(r[i]));
    }
    println!("{ans}");
}

