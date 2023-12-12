#![allow(non_snake_case)]

use std::collections::{HashSet, VecDeque};
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        N:usize,
        Q:usize,
        AB:[(Usize1,Usize1);N-1],
        CD:[(Usize1,Usize1);Q],
    }
    let mut g = vec![vec![];N];
    for (a,b) in AB {
        g[a].push(b);
        g[b].push(a);
    }

    let dist = dijkstra(&g, 0);
    for (c,d) in CD {
        if dist[c] % 2 == dist[d] % 2 {
            println!("Town");
        } else {
            println!("Road");
        }
    }
}

fn dijkstra(graph: &Vec<Vec<usize>>, start: usize) -> Vec<usize> {
    let mut dist: Vec<usize> = vec![usize::MAX; graph.len()];
    let mut queue = VecDeque::new();
    let mut visited: HashSet<usize> = HashSet::new();

    dist[start] = 0;
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        if visited.contains(&node) {
            continue;
        }

        visited.insert(node);

        for &neighbor in &graph[node] {
            if dist[neighbor] == usize::MAX {
                dist[neighbor] = dist[node] + 1;
                queue.push_back(neighbor);
            }
        }
    }

    dist
}
