#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        S:Chars,
    }

    let mut count = vec![0usize; 26];
    for &c in &S {
        let cc = (c as u8 - 'a' as u8) as usize;
        count[cc] += 1;
    }

    // println!("{:?}", count);
    let mut x = 0;
    let mut has_same = false;

    for i in 0..26 {
        for j in i+1..26{
                x += count[i] * count[j];
        }

        if count[i] > 1 {
            has_same = true;
        }
    }

    if has_same {
        x = x + 1;
    }

    println!("{x}");
}
