use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        N: usize,
        mut S:Chars,
    }

    let mut ans = 0;

    S.sort();

    for i in 0..10000000usize {
        let mut sqrts: Vec<char> = format!("{:01$}",i * i,N).chars().collect();
        // println!("{:?}",sqrts);
        sqrts.sort();

        if S == sqrts {
            ans += 1;
        }
    }

    println!("{}", ans);
}
