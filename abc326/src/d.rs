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
    rows.dedup();

    let mut ans = vec![];
    if dfs(n, &row, &col, &rows, 0, &mut ans) {
        println!("Yes");
        for i in 0..n {
            let s = ans[i].iter().join("");
            println!("{}",s);
        }
    } else {
        println!("No");
    }
}


fn dfs(
    n: usize,
    row: &Vec<char>,
    col: &Vec<char>,
    rows: &Vec<Vec<char>>,
    now: usize,
    ans: &mut Vec<Vec<char>>,
) -> bool {
    if now == n {
        // rowはABCが必ず1つずつを満たすので、colをチェックする
        for j in 0..n {
            let mut count = vec![0; 3];
            for i in 0..n {
                if ans[i][j] == '.' {
                    continue;
                }
                count[(ans[i][j] as u8 - b'A') as usize] += 1;
            }

            if !count.iter().all(| &cnt| cnt == 1) {
                return false;
            }
        }
        return true;
    }

    for r in rows.iter() {
        if *r.iter().find(|c| **c != '.').unwrap() != row[now] {
            continue;
        }
        ans.push(r.clone());
        let mut ok = true;
        for j in 0..n {
            for i in 0..=now {
                if ans[i][j] == '.' {
                    continue;
                }

                if ans[i][j] == col[j] {
                    break;
                } else {
                    ok = false;
                }
            }
        }

        if ok && dfs(n, row, col, rows, now + 1, ans) {
            return true;
        }

        ans.pop();
    }

    false
}
