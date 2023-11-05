use proconio::input;

fn main() {
    input! {
        n:usize,
        x:usize,
    }

    let mut dp = vec![vec![]; 100001];
    dp[0].push(x);
    for i in 1..=n {
        input! {
            l:usize,
            a:[usize; l],
        }

        for j in 0..dp[i-1].len() {
            let wari = dp[i-1][j];
            for k in 0..l {
                if wari % a[k] == 0 {
                    dp[i].push(wari / a[k]);
                }
            }
        }

        // println!("{:?}", dp[i]);
    }

    let ans = dp[n].iter().fold(0usize, |acc,&x| acc + if x == 1 { 1 } else { 0 } );
    println!("{}",ans);
}
