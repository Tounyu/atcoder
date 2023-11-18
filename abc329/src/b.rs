use std::collections::BTreeSet;
use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[usize;n],
    }
    a.sort();
    let mut set = BTreeSet::new();
    for a in a {
        set.insert(a);
    }
    set.pop_last();
    let ans = set.pop_last().unwrap();
    println!("{ans}" );
}
