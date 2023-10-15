use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        N:isize,
        T:Chars,
    }

    let mut rmemo: Vec<usize> = vec![];
    let mut lmemo: Vec<usize> = vec![];
    let mut ans = 0;
    let t_len = T.len();

    for i in 0..N{
        input! {
            S:Chars,
        }

        let s_len = S.len();

        let mut l = 0;
        let mut r = 0;
        for j in 0..s_len {
            if  l < t_len && T[l] == S[j] {
                l += 1;
            }
            if r < t_len && T[T.len() - 1 - r] == S[s_len - 1 - j] {
                r += 1;
            }
        }

        lmemo.push(l);
        rmemo.push(r);
    }

    lmemo.sort();
    rmemo.sort();

    for l in lmemo {
        let le = t_len - l;
        let ile = lower_bound(&rmemo, le);
        ans += rmemo.len() - ile;
    }

    println!("{}", ans);
}

fn lower_bound(v: &Vec<usize>, val: usize) -> usize {
    let mut l = 0;
    let mut r = v.len();
    while l < r {
        let mid = (l + r) / 2;
        if v[mid] < val {
            l = mid + 1;
        } else {
            r = mid;
        }
    }
    l
}
