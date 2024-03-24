#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        H:usize,
        W:usize,
        M:usize,
        Q:[(usize,Usize1,usize); M],
    }

    let mut usedH = vec![false; H];
    let mut usedW = vec![false; W];
    let mut wc = W;
    let mut hc = H;
    let mut color = vec![0; 200010];
    let mut count = 0;

    for &(t,a,x) in Q.iter().rev() {
        if t == 1 {
            if !usedH[a] {
                usedH[a] = true;
                color[x] += wc;
                count += wc;
                hc -= 1;
            }
        } else {
            if !usedW[a] {
                usedW[a] = true;
                color[x] += hc;
                count += hc;
                wc -= 1;
            }
        }
    }

    let mut ans = vec![];
    color[0] += W*H - count;

    for i in 0..color.len() {
        if  color[i] > 0 {
            ans.push((i, color[i]));
        }
    }

    println!("{}", ans.len());
    for (c, count) in ans {
        println!("{} {}", c, count);
    }
}
