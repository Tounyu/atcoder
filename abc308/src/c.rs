use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        mut ab:[(u128,u128); n],
    }

    let new =  ab.iter().enumerate().sorted_by(|(ai, a), (bi,b)| (b.0 * (a.0 + a.1)).cmp(&(a.0 * (b.0 + b.1))));
    for (i, _ab) in new {
        print!("{} ", i + 1);
    }
    println!();
}
