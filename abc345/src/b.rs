#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        x:i64,
    }

    if x % 10 == 0 {
        println!("{}", x / 10);
    } else {
        let y = x / 10;
        if y == 0 {
            if x > 0 {
                println!("1");
            } else {
                println!("0");
            }
        }else if y < 0 {
            println!("{}", y);
        } else {
            println!("{}", y + 1);
        }
    }
}
