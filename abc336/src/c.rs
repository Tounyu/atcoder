#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
       n:u64,
    }
    if n == 0 {
        println!("0");
        return;
    }
    let mut ans = decimal_to_base5(n - 1);

    ans = ans.replace("4", "8");
    ans = ans.replace("3", "6");
    ans = ans.replace("2", "4");
    ans = ans.replace("1", "2");

    println!("{ans}");
}

fn decimal_to_base5(decimal_number: u64) -> String {
    let mut result = String::new();
    let mut remaining = decimal_number;

    while remaining > 0 {
        let digit = (remaining % 5) as u8;
        result.insert(0, char::from_digit(digit as u32, 10).unwrap());
        remaining /= 5;
    }

    if result.is_empty() {
        result.push('0');
    }

    result
}
