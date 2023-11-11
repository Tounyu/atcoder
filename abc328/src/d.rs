use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
       s:Chars,
    }

    let ans = solve(&s);

    println!("{}", ans);
}

fn solve(s: &[char]) -> String {
    let mut ans = String::new();

    for &c in s {
        ans.push(c);
        if ans.ends_with("ABC") {
            ans.pop();
            ans.pop();
            ans.pop();
        }
    }
    ans
}
