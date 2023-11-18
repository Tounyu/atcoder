use proconio::input;

fn main() {
    input! {
        n:u64,
    }

    let mut sqrt = 0;
    while sqrt * sqrt <= n {
        sqrt += 1;
    }
    sqrt -=1;
    let mut x = 0;
    for i in 1..=sqrt {
        x += n / i;
    }
    let ans = (2 * x) - (sqrt * sqrt);
    println!("{ans}");
}
