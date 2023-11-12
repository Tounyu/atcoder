use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n:usize,
        s:Chars,
    }
    let mut q = String::new();
    let mut l = String::new();
    let mut r = String::new();
    for &c in &s {
        q.push(c);
        while q.ends_with("()") {
            q.pop();
            q.pop();
        }
    }
    while !q.is_empty() {
        if let Some(c) = q.pop() {
            if c == ')' {
                l.push('(');
            } else {
                r.push(')');
            }
        }
    }
    println!("{}{}{}", l, s.into_iter().join(""),r);
}
