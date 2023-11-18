use std::collections::HashSet;
use std::mem::take;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n:usize,
        q:usize,
        c:[Usize1;n],
        que:[(Usize1,Usize1); q],
    }

    let mut boxs = vec![HashSet::new(); n];
    for i in 0..n {
        boxs[i].insert(c[i]);
    }

    for (a, b) in que {

        let mut aa = take(&mut boxs[a]);
        let mut bb = take(&mut boxs[b]);

        let newb = if aa.len() < bb.len() {
            bb.extend(aa);
            bb
        } else {
            aa.extend(bb);
            aa
        };

        boxs[b]= newb;
        println!("{}",  boxs[b].len());
    }
}
