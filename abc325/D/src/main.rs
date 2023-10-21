use std::cmp::Reverse;
use std::collections::BinaryHeap;
use proconio::input;
/*
優先度付きキュー
*/
fn main() {
    input! {
        N:usize,
        mut TD:[(usize,usize); N],
    }
    TD.sort();
    TD.reverse();

    let mut ans = 0;
    let mut current_time = 0;
    let mut pq = BinaryHeap::new();

    while !TD.is_empty() || !pq.is_empty() {
        if pq.is_empty() && !TD.is_empty() {
            let (t, d) = TD.pop().unwrap();
            current_time = t;
            pq.push(Reverse(t + d));
        }
        while !TD.is_empty() && TD.last().unwrap().0 <= current_time {
            let (t, d) = TD.pop().unwrap();
            current_time = t;
            pq.push(Reverse(t + d));
        }

        while let Some(Reverse(end)) = pq.pop() {
            if end >= current_time {
                ans += 1;
                current_time += 1;
                break;
            }
        }
    }

    println!("{}", ans);
}
