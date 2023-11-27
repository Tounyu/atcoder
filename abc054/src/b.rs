#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n:usize,
        m:usize,
        a:[Chars;n],
        b:[Chars;m],
    }

    for i in 0..=n-m {
        for j in 0..=n-m {
            if check(&a,&b,i,j,m) {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}

fn check(a:&Vec<Vec<char>>,b:&Vec<Vec<char>>, i:usize, j:usize, m:usize) -> bool {
    for k in 0..m {
        let aa = &a[i+k][j..j+m];
        let bb = &b[k];
        if aa != bb {
            return false;
        }
    }
    true
}
