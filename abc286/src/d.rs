use proconio::input;

fn main() {
    input! {
        n:usize,
        x:usize,
        ab:[(usize,usize); n],
    }

    let mut visited = vec![vec![false; x + 1]; n];
    // ab.sort_by(|i,j| j.0.cmp(&i.0));

    if dfs(&ab, &mut visited, 0, x){
        println!("Yes");
    } else {
        println!("No");
    }
}

fn dfs(ab:&Vec<(usize,usize)>, visited:&mut Vec<Vec<bool>>, now:usize, x:usize) -> bool {
    if now == ab.len() {
        return false;
    }
    if visited[now][x] {
        return false;
    }
    visited[now][x] = true;

    let (a,b) = ab[now];

    if x % a == 0 && x / a <= b {
        return true;
    }

    for i in 0..=b {
        if x < i * a {
            break;
        }
        if dfs(ab, visited, now + 1, x - i * a) {
            return true;
        }
    }

    return false;
}
