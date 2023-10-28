use std::collections::VecDeque;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n:usize,
        row:String,
        col:String,
    }

    let ans = check(n,  &row, &col).unwrap();

    for row in ans {
        println!("{}", row);
    }
}
fn check(n: usize, r: &str, c: &str) -> Option<Vec<String>> {
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; n]; n];
    let mut counts = [0, 0, 0];

    // 列の制約を満たすようにマス目を埋める
    for i in 0..n {
        let ch = r.chars().nth(i).unwrap();
        let idx = match ch {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            _ => unreachable!(),
        };
        grid[i][i] = ch;
        counts[idx] += 1;
    }

    // 行の制約を満たすようにマス目を埋める
    for i in 0..n {
        let ch = c.chars().nth(i).unwrap();
        let idx = match ch {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            _ => unreachable!(),
        };
        // すでに埋まっている場合はスキップ
        if grid[i][i] == ch {
            continue;
        }
        // 列で同じ文字が使用されていなければ、その文字を使って埋める
        if counts[idx] == 0 {
            grid[i][i] = ch;
            counts[idx] += 1;
        } else {
            // それ以外の場合は、残りの文字を使って埋める
            for j in 0..n {
                if grid[j][i] == '.' {
                    grid[j][i] = ch;
                    break;
                }
            }
        }
    }

    // 制約を満たす書き込み方が存在するか確認
    if counts.iter().all(|&x| x == 1) {
        // 書き込み方が存在する場合、文字列に変換して返す
        let result: Vec<String> = grid.into_iter().map(|row| row.into_iter().collect()).collect();
        Some(result)
    } else {
        // 書き込み方が存在しない場合、Noneを返す
        None
    }
}