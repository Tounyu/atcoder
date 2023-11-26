#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        x:i64,
    }

    let s: Vec<i64> = x.to_string().chars().map(|x| x.to_digit(10).unwrap() as i64).collect();
    if check(&s, s.len()) {
        println!("{}", x);
        return;
    }

    let mut ans = vec![];

    dfs(&s, 0, &mut ans);
    for ans in ans {
        print!("{ans}");
    }
    println!();
}

fn dfs(s:&Vec<i64>, now:usize, ans:&mut Vec<i64>) -> bool {
    let ss = s[..now].iter().join("");
    let aa = ans[..now].iter().join("");
    if ss > aa {
        return false;
    }
    if s.len() == now {
        return check(ans, now);
    }
    if !check(ans, now) {
        return false;
    }

    if now == 0 {
        for i in s[0]..10 {
            ans.push(i);
            if dfs(s, now + 1, ans) {
                return true;
            }
            ans.pop();
        }
    } else {
        for i in 0..10 {
            ans.push(i);
            if dfs(s, now + 1, ans) {
                return true;
            }
            ans.pop();
        }
    }
    false
}

fn check(s:&Vec<i64>, keta:usize) -> bool {
    if keta <= 2 {
        return true;
    }
    let diff = s[1] - s[0];
    for i in 2..keta {
        if s[i] - s[i-1] != diff {
            return false;
        }
    }
    true
}
