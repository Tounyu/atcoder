use proconio::input;

fn main() {
    input! {
        n:usize,
    }

    let mut memo = vec![0usize; 500_000_009];
    memo[0] = 1;
    println!("{}", calc(n, &mut memo));
}

fn calc(n:usize, memo:&mut Vec<usize>) -> usize {
    if n < memo.len() && memo[n] > 0 {
        return memo[n];
    }
    let calc1 = calc(n/2, memo);
    let calc2 = calc(n/3, memo);
    if n < memo.len() {
        memo[n] = calc1 + calc2;
    }
    return calc1 + calc2;
}
