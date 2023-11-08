use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        abc:[(usize,usize,i64); m],
    }

    let mut g = vec![vec![]; n + 1];
    for (a,b,c) in abc{
        g[a].push((b,c));
        g[b].push((a,c));
    }
    let mut ans = 0i64;
    for s in 1..=n{
        let mut visited = vec![false; n + 1];
        ans = ans.max(dfs(&g, &mut visited, s));
    }
    println!("{}", ans);
}

fn dfs(g:&Vec<Vec<(usize,i64)>>, visited:&mut Vec<bool>, now:usize) -> i64{
    if visited[now] {
        return -1000000000i64;
    }
    visited[now] = true;
    let mut max_cost = 0i64;
    for &(next,c) in &g[now] {
        if visited[next] {
            continue;
        }
        let cost = dfs(g, visited, next) + c;
        max_cost = max_cost.max(cost);
    }
    visited[now] = false;
    max_cost
}
