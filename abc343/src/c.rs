#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N:u128,
    }

    for i in (0u128..=1000000).rev(){
        let k =  i * i * i;
        if k <= N {
            let s: Vec<char> = format!("{}", k).chars().collect();
            let rev: Vec<char> = format!("{}",k).chars().rev().collect();
            if s == rev {
                println!("{k}");
                return;
            }
        }
    }
}
