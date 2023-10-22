use std::cmp::{min, Reverse};
use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        a:usize,
        b:usize,
        c:usize,
        d:[[usize; n];n],
    }

    // (next, cost)
    let mut g1 = vec![vec![]; n];
    let mut g2 = vec![vec![]; n];
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            g1[i].push((j, d[i][j] * a));
            g2[i].push((j, d[i][j] * b + c));
        }
    }
    // 車は前からのコストを取る
    let dist1 = dijkstra(&g1, 0);
    // 電車は後ろからのコストを取る
    let dist2 = dijkstra(&g2, n - 1);
    let mut ans = dist1[0] + dist2[0];
    for i in 1..n {
        ans = min(ans, dist1[i] + dist2[i]);
    }

    println!("{}", ans);
}

fn dijkstra(g: &Vec<Vec<(usize, usize)>>, v: usize) -> Vec<usize> {
    let mut pq = BinaryHeap::new();
    let inf = 1usize << 60;
    let mut dist = vec![inf; g.len()];
    dist[v] = 0usize;
    pq.push((Reverse(dist[v]), v));

    while !pq.is_empty() {
        if let Some((Reverse(now_cost), now)) = pq.pop() {
            for &(next, cost) in &g[now] {
                if dist[next] > dist[now] + cost {
                    dist[next] = dist[now] + cost;
                    pq.push((Reverse(dist[next]), next));
                }
            }
        }
        // println!("{:?}", dist);
    }
    dist
}
