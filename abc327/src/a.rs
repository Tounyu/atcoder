use proconio::input;

fn main() {
    input! {
        _n:usize,
        s:String,
    }

    if s.contains("ab") || s.contains("ba") {
        println!("Yes");
        return;
    }
    println!("No");
}
