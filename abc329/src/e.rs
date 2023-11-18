use std::collections::VecDeque;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n:usize,
        m:usize,
        mut s:Chars,
        t:Chars,
    }

    let mut used = vec![false; n];
    let mut q = VecDeque::new();

    for i in 0..n-m + 1 {
        push(i, &mut s, &t, &mut used, &mut q, n,m);
    }

    while !q.is_empty() {
        let i = q.pop_front().unwrap();
        for j in 0..m {
            s[i + j] = '#';
        }

        let l = if i + 1 > m {
            i-m+1
        } else {
            0
        };

        for ni in l..=i+m-1 {
            push(ni,  &mut s, &t, &mut used, &mut q, n,m);
        }
    }

    if s == vec!['#'; n] {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn push(i:usize, s:&mut[char], t:&[char], used:&mut[bool], q:&mut VecDeque<usize>,  n:usize, m:usize) {
    if i < 0 || i + m > n {
        return;
    }
    if used[i] {
        return;
    }
    for j in 0..m {
        if s[i + j] != '#' && s[i + j] != t[j] {
            return;
        }
    }
    used[i] = true;
    q.push_back(i);
}
