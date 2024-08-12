#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        H:usize,
        W:usize,
        N:usize,
        T:Chars,
        S:[Chars;H],
    }

    let mut ans = 0;
    for i in 1..H-1 {
        for j in 1..W-1 {
            if check(&S,&T,i,j) {
                // println!("i={},j={}",i+1,j+1);
                ans += 1;
            }
        }
    }
    println!("{ans}");
}

fn check(S:&Vec<Vec<char>>, T:&Vec<char>, i:usize, j:usize) -> bool {
    let mut i = i;
    let mut j = j;
    if S[i][j] == '#' {
        return false;
    }

    for &t in T {
        if t == 'L' {
            j -=1;
        }
        if t == 'R' {
            j +=1;
        }
        if t == 'U' {
            i -= 1;
        }
        if t == 'D' {
            i += 1;
        }

        if S[i][j] == '#' {
           return false;
        }
    }
    return true;
}
