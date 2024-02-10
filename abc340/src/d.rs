#![allow(non_snake_case)]

use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet, VecDeque};
use proconio::input;
use proconio::marker::Usize1;

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

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Each node is represented as a `usize`, for a shorter implementation.
#[derive(Clone)]
struct Edge {
    node: usize,
    cost: usize,
}

fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(State { cost: 0, position: start });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal { return Some(cost); }

        // Important as we may have already found a better way
        if cost > dist[position] { continue; }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position] {
            let next = State { cost: cost + edge.cost, position: edge.node };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}

// fn dijkstra(graph: &Vec<Vec<(usize, usize)>>, start: usize) -> Vec<usize> {
//     let mut dist: Vec<usize> = vec![usize::MAX; graph.len()];
//     let mut queue = VecDeque::new();
//     let mut visited: HashSet<usize> = HashSet::new();
//
//     dist[start] = 0;
//     queue.push_back(start);
//
//     while let Some(node) = queue.pop_front() {
//         if visited.contains(&node) {
//             continue;
//         }
//
//         visited.insert(node);
//
//         for &(neighbor, cost) in &graph[node] {
//             if dist[neighbor] == usize::MAX {
//                 dist[neighbor] = dist[node] + cost;
//                 queue.push_back(neighbor);
//             }
//         }
//     }
//
//     dist
// }
