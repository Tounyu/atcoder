use std::collections::BTreeSet;
use ac_library::Dsu;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n:usize,
        m:usize,
        uv:[(Usize1, Usize1);m],
        k:usize,
        xy:[(Usize1, Usize1);k],
        q:usize,
        pq:[(Usize1, Usize1);q],
    }

    let mut dsu = Dsu::new(n * 2);
    for (u,v) in uv {
        dsu.merge(u,v);
    }
    let mut set = BTreeSet::new();
    for (x, y) in xy {
        let xx = dsu.leader(x);
        let yy = dsu.leader(y);
        set.insert((xx.min(yy), xx.max(yy)));
    }
    for (p,q) in pq{
        let pp = dsu.leader(p);
        let qq = dsu.leader(q);
        if pp != qq && set.contains(&(pp.min(qq), pp.max(qq))) {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}
