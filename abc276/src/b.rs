use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        ab:[(usize,usize); m],
    }
    let mut v = vec![vec![]; n + 1];
    for (a,b) in ab{
        v[a].push(b);
        v[b].push(a);
    }
    for i in 1..=n {
        v[i].sort();
        let s = v[i].iter().join(" ");
        println!("{} {}", v[i].len(), s);
    }
}
