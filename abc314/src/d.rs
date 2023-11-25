#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n:usize,
        s:Chars,
        q:usize,
        Q:[(usize,usize,char);q],
    }

    let mut lower = None;
    let mut ss = vec![(0usize, '-'); n];
    for i in 0..n {
        ss[i].1 = s[i];
    }

    for (i ,&(t, x, c)) in Q.iter().enumerate() {
        if t == 1 {
            ss[x-1] = (i, c);
        }
        if t == 2 {
            lower = Some((i,true));
        }
        if t == 3 {
            lower = Some((i,false));
        }
    }

    for i in 0..n {
        if let Some((time, is_lower)) = lower {
            if time < ss[i].0 {
                print!("{}", ss[i].1);
            } else {
                if is_lower {
                    print!("{}", ss[i].1.to_lowercase());
                } else {
                    print!("{}", ss[i].1.to_uppercase());
                }
            }
        } else {
            print!("{}", ss[i].1);
        }
    }
    println!();
}
