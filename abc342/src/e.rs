#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        N:usize,
        M:usize,
        train:[(usize,usize,usize,usize,Usize1,Usize1);M],
    }

    let mut g = vec![vec![]; N];

    for (l,d,k,c,A,B) in train {
        for dd in (l..=(l+ (k-1)*d)).step_by(d) {
            g[A].push((B, dd, c));
        }
    }

    for i in 0..N{
        println!("{:?}",g[i]);
    }

    for s in 0..N-1 {
        let mut visited = vec![false; N];
        visited[s] = true;
        let mut ans = -1;
        for &(next, time, cost) in &g[s] {
            if dfs(&g,&mut visited, next, time+cost, N-1) {
                ans = ans.max(time as i64);
            }
        }
        if ans < 0 {
            println!("Unreachable");
        } else {
            println!("{ans}");
        }
    }
}

fn dfs(g:&Vec<Vec<(usize,usize,usize)>>, visited: &mut Vec<bool>, now:usize, now_t:usize, goal:usize) -> bool {
    if now == goal {
        return true;
    }
    if visited[now] {
        return false;
    }
    visited[now] = true;

    for &(next, time, cost) in &g[now] {
        if time < now_t { continue; }
        if dfs(g, visited, next, time+cost, goal) {
            return true;
        }
    }

    visited[now] = false;
    return false;
}
