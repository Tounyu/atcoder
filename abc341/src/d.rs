#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N:u64,
        M:u64,
        K:u64,
    }

    let ll = lcm(N,M);
    let mut r = 2 * 10u64.pow(18);
    let mut l = 0;
    let mut mid = (l + r) / 2;

    while l + 1 < r {
        mid = (l + r) / 2;
        let c = (mid/N) + (mid/M) - 2 * (mid/ll);
        if c < K {
            l = mid;
        } else {
            r = mid;
        }
    }

    println!("{}", r);
}

fn lcm(a:u64, b:u64)->u64 {
    return a/gcd(a,b) * b;
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
