use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n:usize,
        s:[Chars;n],
    }
    let mut ss = vec![];
    for s in s {
        if !['H','D','C','S'].contains(&s[0]) {
            println!("No");
            return;
        }
        if !['A' , '2' , '3' ,'4' , '5' , '6' , '7' , '8' , '9' , 'T' , 'J' , 'Q' , 'K'].contains(&s[1]) {
            println!("No");
            return;
        }
        let str = s.iter().join("");
        if ss.contains(&str) {
            println!("No");
            return;
        }
        ss.push(str);
    }
    println!("Yes");
}
