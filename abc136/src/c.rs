use proconio::input;

fn main() {
    input! {
        n:usize,
        h:[usize;n],
    }
    let mut prev = h[0];
    for i in 1..n {
        if prev > h[i] {
            println!("No");
            return;
        }

        if prev <= h[i] - 1 {
            // 1減らしてもいい
            prev = h[i] - 1;
        } else {
            prev = h[i];
        }
    }
    println!("Yes");
}
