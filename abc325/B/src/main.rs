use std::cmp::max;
use proconio::input;

fn main() {
    input! {
        N:usize,
        WX:[(usize, usize); N],
    }

    let mut time = vec![0usize; 48];

    for (w, x) in WX {
        let start = (x + 9);
        let end = (x + 18);
        for t in start..end {
            time[t % 24] += w;
        }
    }

    let max = time.iter().fold(0, |acc,&x| max(acc, x));

    println!("{}", max);
}
