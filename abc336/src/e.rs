#![allow(non_snake_case)]

use std::collections::HashSet;
use proconio::input;

fn main() {
    input! {
        n:u64,
    }


    let result = count_good_numbers(n);
    println!("{}", result);
}
