#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N:usize,
        H:usize,
        W:usize,
        AB:[(usize,usize); N],
    }

    let mut used = vec![vec![0i32; W]; H];
    let result = can_fill(&AB, 0, &mut used, H, W);
    if result {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn can_fill(tiles: &Vec<(usize, usize)>, now:usize, used: &mut Vec<Vec<i32>>, H:usize, W:usize) -> bool {
    if now == tiles.len() - 1 {
        let mut x = 0;
        let mut y = 0;
        for i in 0..H {
            for j in 0..W {
                x += used[i][j];
                y += used[i][j];
            }
        }
    }


    for &(a, b) in tiles.iter() {
        if !used[a][b] && a <= h && b <= w {
            for i in 0..a {
                for j in 0..b {
                    used[i][j] = true;
                }
            }
            if can_fill(tiles, used, h - a, w) || can_fill(tiles, used, a, w - b) {
                return true;
            }
            for i in 0..a {
                for j in 0..b {
                    used[i][j] = false;
                }
            }
        }
        if !used[b][a] && a <= w && b <= h && a != b {
            for i in 0..b {
                for j in 0..a {
                    used[i][j] = true;
                }
            }
            if can_fill(tiles, used, h - b, w) || can_fill(tiles, used, h, w - a) {
                return true;
            }
            for i in 0..b {
                for j in 0..a {
                    used[i][j] = false;
                }
            }
        }
    }

    false
}