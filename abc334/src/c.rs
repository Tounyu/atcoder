#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N:usize,
        k:usize,
        mut a:[usize;k],
    }

    a.sort();

    if a.len() == 1 {
        println!("0");
        return;
    }

    if a.len() % 2 == 0 {
        let mut ans = 0;
        for i in (0..k).step_by(2) {
            ans += a[i + 1] - a[i];
        }
        println!("{}", ans);
        return;
    }

    let mut asc = vec![];
    asc.push(0);
    let mut desc = vec![];
    desc.push(0);
    let mut sum = 0;
    for i in (0..k).step_by(2) {
        if i + 1 >= k {
            continue;
        }
        sum += a[i + 1] - a[i];
        asc.push(sum);
    }
    // println!("{:?}", asc);
    let mut sum = 0;
    for i in (1..k).rev().step_by(2) {
        sum += a[i] - a[i - 1];
        desc.push(sum);
    }
    // println!("{:?}", desc);
    let mut ans = usize::MAX;
    for i in 0..asc.len() {
        let v = asc[i] + desc[asc.len()-1-i];
        // println!("{v}");
        ans = ans.min(v);
    }
    println!("{ans}");
}
