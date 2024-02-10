#![allow(non_snake_case)]

use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet, VecDeque};
use proconio::input;
use proconio::marker::Usize1;

// dijkstra ダイクストラ法
fn main() {
    input! {
        N:usize,
        X:[(usize,usize,Usize1);N-1],
    }

    let mut g = vec![vec![]; N];

    for (i, &(a,b,x)) in X.iter().enumerate() {
        g[i].push(Edge { node: i + 1, cost: a });
        if i != x {
            g[i].push(Edge { node: x, cost: b });
        }
    }

    // println!("{:?}",g);

    let d = shortest_path(&g, 0, N-1).unwrap();
    // let dist = d.distance(N-1);

    println!("{}", d);
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone)]
struct Edge {
    node: usize,
    cost: usize,
}

fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State { cost: 0, position: start });

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal { return Some(cost); }
        if cost > dist[position] { continue; }

        for edge in &adj_list[position] {
            let next = State { cost: cost + edge.cost, position: edge.node };

            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }
    }
    None
}
