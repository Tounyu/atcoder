use std::cmp::min;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        N:usize,
        mut t:Chars,
    }

    let mut ans = vec![];

    for i in 0..N {
        input! {
           mut s:Chars,
        }

        if s.len().abs_diff(t.len()) > 1 {
            continue;
        }

        if s.len() == t.len() {
            let mut count = 0;
            for j in 0..s.len() {
                if s[j] != t[j] {
                    count += 1;
                }
                if count > 1 {
                    break;
                }
            }
            if count > 1 {
                continue;
            }
        } else {
            let n = min(s.len(), t.len());
            if t.len() > s.len(){
                let mut count = 0;
                for j in 0..n {
                    if s[j] != t[j + count] {
                        count += 1;
                        if count > 1 {
                            break;
                        }
                        if s[j] != t[j+1] {
                            count += 1;
                            break;
                        }
                    }
                }
                if count > 1 {
                    continue;
                }
            } else {
                let mut count = 0;
                for j in 0..n {
                    if s[j + count] != t[j]{
                        count += 1;
                        if count > 1 {
                            break;
                        }
                        if s[j + 1] != t[j] {
                            count += 1;
                            break;
                        }
                    }
                }
                if count > 1 {
                    continue;
                }
            }
        }
        ans.push(i + 1);
    }

    ans.sort();

    println!("{}", ans.len());

    if ans.len() > 0 {
        for i in 0..ans.len() {
            if i == 0 {
                print!("{}", ans[i]);
            } else {
                print!(" {}", ans[i]);
            }
        }
    }
    println!();
}
