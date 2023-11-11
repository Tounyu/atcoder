use ac_library::Dsu;
use itertools::Itertools;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n:usize,
        m:usize,
        k:u64,
        edge:[(Usize1,Usize1,u64);m],
    }

    let mut ans = k;

    for comb in (0..m).combinations(n-1){
        let mut dsu = Dsu::new(n);
        let mut cost = 0;
        for i in comb{
            let (u,v,w) = edge[i];
            dsu.merge(u, v);
            cost += w;
            cost %= k;
        }

        if (0..n).fold(true, |acc, i| acc && dsu.same(0, i)) {
            ans = ans.min(cost);
        }
    }

    println!("{ans}");
}
