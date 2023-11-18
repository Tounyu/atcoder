use proconio::input;
use proconio::marker::Chars;


fn main() {
    input! {
        n:usize,
        s:Chars,
    }

    let mut v = vec![0usize; 50];
    let mut count = 1;
    let mut now = s[0];
    v[(s[0] as usize) - ('a' as usize)] = count;

    for i  in 1..n {
        let j = (s[i] as usize) - ('a' as usize);
        if now == s[i] {
            count += 1;
        } else {
            count = 1;
            now = s[i];
        }
        v[j] = v[j].max(count);
    }

    let ans = v.iter().fold(0usize, |acc, &v| acc + v);

    println!("{}", ans);
}
