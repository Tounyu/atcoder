use proconio::input;

fn main() {
    input! {
        n:usize,
        c:i64,
        abc:[(usize,usize,i64);n],
    }
    let mut memo = vec![];
    for (a, b, cost) in abc {
        memo.push((a - 1, cost));
        memo.push((b, -cost));
    }
    memo.sort();
    let mut total = 0;
    let mut now = 0;
    let mut now_cost = 0;
    for (i, cost) in memo {
        if i != now {
            total += (i - now) as i64 * c.min(now_cost);
            now = i;
        }
        now_cost += cost;
    }
    println!("{total}");
}
