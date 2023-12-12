#![allow(non_snake_case)]

use std::collections::VecDeque;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        N:usize,
        AB:[(Usize1,Usize1);N-1],
    }

    let mut g =vec![vec![];N];
    for (a,b) in AB {
        g[a].push(b);
        g[b].push(a);
    }
    for i in 0..N {
        g[i].sort();
    }

    let mut visited = vec![false;N];
    dfs(&g, 0, &mut visited);
}

fn dfs(g:&Vec<Vec<usize>>, now:usize, visited:&mut Vec<bool>) -> bool {
    if visited[now] {
        return false;
    }
    visited[now] = true;
    print!("{} ", now + 1);

    for &next in &g[now] {
        if dfs(g,next,visited) {
            print!("{} ", now + 1)
        }
    }
    true
}
