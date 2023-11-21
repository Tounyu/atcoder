use proconio::input;
use proconio::marker::Usize1;

// dfs
fn main() {
    input! {
        n:usize,
        x:Usize1,
        y:Usize1,
        uv:[(Usize1, Usize1); n-1],
    }
    let mut g = vec![vec![]; n];
    let mut v = vec![false; n];
    let mut ans = vec![];
    for (u,v) in uv {
        g[u].push(v);
        g[v].push(u);
    }
    ans.push(x);
    dfs(&g, &mut v,x,y, &mut ans);
    for ans in ans {
        print!("{} ", ans+1);
    }
    println!();
}

fn dfs(g:&Vec<Vec<usize>>,v:&mut[bool],now:usize,goal:usize, ans:&mut Vec<usize>) -> bool {
    if now == goal {
        return true;
    }
    if v[now] {
        return false;
    }
    v[now] = true;
    for &next in &g[now] {
        ans.push(next);
        if dfs(g,v,next,goal,ans) {
            return true;
        }
        ans.pop();
    }
    return false;
}
