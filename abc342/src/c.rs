#![allow(non_snake_case)]

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        N:usize,
        S:Chars,
        Q:usize,
        CD:[(char,char);Q],
    }

    let mut conv = vec![0usize; 26];
    for i in 0..26 {
        conv[i] = i;
    }
    for (c,d)in CD {
        let cc = (c as u8 - 'a' as u8) as usize;
        let dd = (d as u8 - 'a' as u8) as usize;

        for i in 0..26 {
            if conv[i] == cc {
                conv[i] = dd;
            }
        }
    }

    for i in 0..N {
        let ss =  (S[i] as u8 - 'a' as u8) as usize;
        print!("{}", (conv[ss] as u8 + 'a' as u8) as char);
    }
    println!();
}
