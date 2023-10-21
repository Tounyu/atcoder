use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        H:usize,
        W:usize,
        S:[Chars; H],
    }

    let mut visited = vec![vec![false; W]; H];
    let mut max = 0usize;

    for i in 0..H {
        for j in 0..W {
            if S[i][j] == '#' && !visited[i][j] {
                check_link(&S, &mut visited, j as i64, i as i64, W, H);
                max += 1;
            }
        }
    }

    println!("{}", max);
}

const DIRECTIONS: [(i64, i64); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

fn check_link(S:&Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, x:i64, y:i64, w:usize, h:usize)  {
    if y < 0 || (h as i64) <= y || x < 0 || (w as i64) <= x {
        return;
    }
    if visited[y as usize][x as usize] {
        return;
    }
    if S[y as usize][x as usize] == '.'{
        return;
    }

    visited[y as usize][x as usize] = true;

    // 8方向を探索
    for &(dirx, diry) in &DIRECTIONS {
        check_link(S, visited, x + dirx, y + diry, w, h);
    }

    return;
}
