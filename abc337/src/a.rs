#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N:usize,
        xy:[(usize,usize);N],
    }
    let mut xx= 0;
    let mut yy = 0;
    for (x,y) in xy {
        xx += x;
        yy += y;
    }
    if xx == yy {
        println!("Draw");
        return;
    }
    if xx > yy {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}
