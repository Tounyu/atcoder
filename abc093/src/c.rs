use proconio::input;

fn main() {
    input! {
        a:usize,
        b:usize,
        c:usize,
    }

    let mut v = vec![a,b,c];
    v.sort();

    let mut ans = 0;
    while check(&v) < 0 {
        v[0] += 2;
        v.sort();
        ans += 1;
    }

    println!("{}", ans + check(&v));
}

fn check(v:&Vec<usize>) -> i32 {
    if v[0] == v[1] && v[1] == v[2] {
        return 0;
    }
    if v[0] == v[1] && v[2] == v[0] + 1 {
        return 1;
    }
    if v[2] == v[1] && v[2] == v[0] + 2 {
        return 1;
    }
    return -1;
}
