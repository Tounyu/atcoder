use std::collections::HashMap;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n:usize,
        m:usize,
        a:[Usize1;m],
    }

    let mut memo = HashMap::new();
    let mut cur = 0;
    let mut cur_count = 0;

    for (i, &vote) in a.iter().enumerate() {
        if let  Some(&count) = memo.get(&vote) {
            memo.insert(vote, count + 1);
            if cur_count < count + 1 {
                cur = vote;
                cur_count = count + 1;
            } else if cur_count == count + 1 {
                cur = cur.min(vote);
            }
        } else {
            memo.insert(vote, 1);
            if cur_count < 1 {
                cur = vote;
                cur_count = 1;
            } else if cur_count == 1 {
                cur = cur.min(vote);
                cur_count = 1;
            }
        }

        println!("{}", cur + 1);
    }
}
