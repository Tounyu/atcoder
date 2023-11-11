use proconio::input;

fn main() {
    input! {
        n:usize,
        d:[usize;n],
    }
    let mut ans = 0;
    for i in 1..=n {
        if let Some(x) = check(i) {
            for j in 1..=d[i - 1] {
                if let Some(y) = check(j) {
                    if x == y {
                        ans += 1;
                    }
                }
            }
        }
    }
    println!("{ans}");
}

fn check(n: usize) -> Option<u32> {
    let digits: Vec<u32> = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let first = digits[0];

    for digit in digits {
        if digit != first {
            return None;
        }
    }

    Some(first)
}
