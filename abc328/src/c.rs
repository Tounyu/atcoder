use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n:usize,
        q:usize,
        s:Chars,
        lr:[(usize,usize);q],
    }

    let mut memo = vec![0usize; n];
    let mut sum = 0;
    for i in 1..n {
        if s[i] == s[i - 1] {
            sum += 1;
        }
        memo[i] = sum;
    }

    for (l,r) in lr {
        let mut ll = if l < 1 { 0 } else {l - 1};
        println!("{}", memo[r - 1] - memo[ll]);
    }
}
