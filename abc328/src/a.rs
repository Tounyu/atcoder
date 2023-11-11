use proconio::input;

fn main() {
    input! {
        n:usize,
        x:usize,
        s:[usize;n],
    }
    let mut ans = 0usize;
    for s in s{
        if s <= x {
            ans += s;
        }
    }
    println!("{ans}");
}
