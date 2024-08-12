#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        H:usize,
        W:usize,
        N:usize,
    }

    let mut ans = vec![vec!['.'; W]; H];
    let mut x = 0;
    let mut y = 0;
    let mut dir = 0;

    for i in 0..N {
        if ans[y][x] == '.' {
            ans[y][x] = '#';
            dir = (dir + 1) % 4;
        } else {
            ans[y][x] = '.';
            dir = (dir + 3) % 4;
        }

        if dir == 0 {
            y = (y + H - 1) % H;
        } else if dir == 1 {
            x = (x + 1) % W;
        } else if dir == 2 {
            y = (y + 1) % H;
        } else {
            x = (x + W - 1) % W;
        }
    }

    for i in 0..H {
        for j in 0..W {
            print!("{}", ans[i][j]);
        }
        println!();
    }
}
