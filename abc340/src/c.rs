#![allow(non_snake_case)]

use std::collections::VecDeque;
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N:usize,
    }

    let mut s = VecDeque::new();
    let mut ans = 0;
    s.push_back((N, 1));
    while s.len() > 0 {
        let (x, c) = s.pop_front().unwrap();
        if x > 1 {
            let a = x / 2;
            if x % 2 == 0 {
                if let Some(i) = s.iter().position(|&t| t.0 == a) {
                    s[i].1 += c * 2;
                } else {
                    s.push_back((a, c*2));
                }
            } else {
                if let Some(i) = s.iter().position(|&t| t.0 == a) {
                    s[i].1 += c;
                } else {
                    s.push_back((a, c));
                }
                if let Some(i) = s.iter().position(|&t| t.0 == a + 1) {
                    s[i].1 += c;
                } else {
                    s.push_back((a+1, c));
                }
            }
            ans += x * c;
        }
    }
    println!("{ans}");
}
