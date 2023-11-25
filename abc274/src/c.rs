use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize; n],
    }
    let mut ans = vec![0usize; n * 2 + 1];
    for i in 0..n {
        ans[2 * i + 1] = ans[a[i] - 1] + 1;
        ans[2 * i + 2] = ans[a[i] - 1] + 1;
    }
    for ans in ans {
        println!("{}", ans);
    }
}
