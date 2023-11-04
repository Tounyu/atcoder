use proconio::input;

fn main() {
    input! {
        n:i64,
        d:i64,
        w:i64,
        mut tx:[(i64, i64); n],
    }

    tx.sort_by_key(| &x| x.0);

    // 遅延セグ木で

    println!("{}", m);
}
