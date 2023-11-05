use std::collections::VecDeque;
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s:Chars,
        n:usize,
    }

    let mut v = VecDeque::new();
    let mut asc = true;

    for s in s {
        v.push_back(s);
    }

    for _i in 0..n {
        input! {
            q:usize,
        }
        if q == 1 {
            asc = !asc;
        }
        if q == 2 {
            input! {
                f:usize,
                c:char,
            }
            if f == 1 {
                if asc {
                    v.push_front(c);
                } else {
                    v.push_back(c);
                }
            }
            if f == 2 {
                if asc {
                    v.push_back(c);
                } else {
                    v.push_front( c);
                }
            }
        }
    }
    let  ans = if asc {
        v.iter().join("")
    } else {
        v.iter().rev().join("")
    };
    println!("{ans}");
}
