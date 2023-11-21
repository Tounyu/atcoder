use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        k:usize,
        a:[usize;n],
        b:[usize;m],
    }

    let mut sum_a = vec![0usize;n + 1];
    let mut sum_b = vec![0usize;m + 1];
    for i in 1..=n {
        sum_a[i] = sum_a[i-1] + a[i-1];
    }
    for i in 1..=m {
        sum_b[i] = sum_b[i-1] + b[i-1];
    }

    let mut ans = 0;
    let mut bi = m as isize;
    for ai in 0..=n {
        while 0 <= bi && k < sum_a[ai] + sum_b[bi as usize] {
            bi -= 1;
        }
        if 0 <= bi {
            ans = ans.max(ai + bi as usize);
        }
    }
    println!("{}", ans);
}
