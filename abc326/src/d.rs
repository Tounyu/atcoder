use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n:usize,
        row:Chars,
        col:Chars,
    }

    let mut rows = vec![];

    let mut chars = vec!['A','B','C'];
    for _i in 0..n-3{
        chars.push('.');
    }
    for r in chars.into_iter().permutations(n) {
        rows.push(r);
    }

    let mut index = vec![0usize; n];
    if let Some(v) = dfs(&rows, 0, &row, &col, &mut index) {
        println!("Yes");
        for row in v {
            println!("{}", row.iter().join(""));
        }
    } else {
        println!("No");
    }
}

fn dfs(rows:&Vec<Vec<char>>, now:usize, row:&Vec<char>, col:&Vec<char>, index:&mut Vec<usize>) -> Option<Vec<Vec<char>>> {
    let mut v = vec![vec![]; now];
    for i in 0..now {
        for &c in &rows[index[i]] {
            v[i].push(c);
        }
    }

    if now > 0 {

        let n = row.len();
        if now == n {
            let mut check = vec!['A','B','C'];
            for _i in 0..n-3{
                check.push('.');
            }
            check.sort();
            for x in 0..n {
                let mut cy = vec![];
                for y in 0..n {
                    cy.push(v[y][x]);
                }
                cy.sort();
                if cy != check {
                    return None;
                }
            }
        }

        let mut cx = vec![];
        let mut cy = vec![];
        for x in 0..now {
            for y in 0..now {
                if v[y][x] != '.' {
                    cx.push(v[y][x]);
                    break;
                }
            }
        }

        if cx != col[..now] {
            return None;
        }
        for y in 0..now {
            for x in 0..now {
                if v[y][x] != '.' {
                    cy.push(v[y][x]);
                    break;
                }
            }
        }
        if cy != row[..now] {
            return None;
        }

        if now == n {
            return Some(v);
        }
    }

    for i in 0..rows.len() {
        index[now] = i;
        // println!("{:?}",index);
        if let Some(v) = dfs(
            rows,
            now + 1,
            row,
            col,
            index
        ) {
            return Some(v);
        }
    }

    return None;
}
