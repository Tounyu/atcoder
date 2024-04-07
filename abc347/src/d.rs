#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        a:usize,
        b:usize,
        C:usize,
    }

    let cc:Vec<char> = format!("{:060b}", C).chars().collect();
    println!("{:?}", cc);
    if !dfs(0, 0, 0, 0,0, C,a,b, 60) {
        println!("-1");
    }
}

fn dfs(now:usize, X:usize, Y:usize, xc:usize, yc:usize, c:usize, a:usize,b:usize,goal:usize) -> bool {
    println!("{now}, X={X},Y={Y}");
    let nn = goal - now-1;

    if c % 2usize.pow(now as u32) != X ^ Y {
       return  false;
    }

    if xc > a { return false; }
    if yc > b { return false; }

    if nn == 0 {
        if X ^ Y == c && xc == a && yc == b {
            println!("{} {}", X,Y);
            return true;
        } else {
            return false;
        }
    }
    let cc = (c >> now) %2;
    if cc == 1 {
        if dfs(now+1, X + (1 << now), Y, xc+1, yc, c, a,b,goal) {
            return true;
        } else if dfs( now+1, X , Y + (1 << now),xc, yc+1, c, a,b,goal) {
            return true;
        } else {
            return false;
        }
    } else {
        if dfs( now+1, X + (1 << now), Y + (1 << now),xc+1, yc+1, c, a,b,goal) {
            return true;
        } else if dfs(now + 1, X, Y,xc, yc, c, a,b,goal) {
            return true;
        } else {
            return false;
        }
    }
}
