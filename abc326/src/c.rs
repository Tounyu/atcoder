use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        mut a:[usize; n],
    }

    a.sort();

    let mut ans = 0;
    let mut l = 0;
    let mut r = 0;
    while r < n {
        while r < n && a[r] - a[l] < m {
            r += 1;
        }
        ans = ans.max(r-l);
        l += 1;
    }
    println!("{}", ans);
}
