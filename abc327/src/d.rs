use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        a:[usize;m],
        b:[usize;m],
    }

    let mut g = vec![vec![];n + 1];

    for i in 0..m {
        if a[i] == b[i] {
            println!("No");
            return;
        }

        g[a[i]].push(b[i]);
        g[b[i]].push(a[i]);
    }

    let mut memo = vec![-1i32; n + 1];
    let mut visited = vec![false; n + 1];

    let mut ans = true;
    for a in a {
        if !visited[a] {
            let res = check(&g, &mut visited, &mut memo, a, 1);
            ans &= res;
        }
    }
    for b in b {
        if !visited[b] {
            let res = check(&g, &mut visited, &mut memo, b, 1);
            ans &= res;
        }
    }

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn check(g:&Vec<Vec<usize>>, visited: &mut Vec<bool>, x: &mut Vec<i32>, now:usize, color:i32) -> bool {
    // println!("{:?}", x);

    if visited[now] {
        return  x[now] == color;
    }

    visited[now] = true;
    x[now] = color;

    for &next in &g[now] {
        let res = check(g,visited,x, next, 1 - color); // colorの1,0を反転
        if !res {
            return false;
        }
    }
    return true;
}
