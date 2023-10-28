use proconio::input;

fn main() {
    input! {
        x:i64,
        y:i64,
    }
    let s = x - y;

    if s < 0 {
        if s.abs() > 2 {
            println!("No");
        } else {
            println!("Yes");
        }
    } else {
        if s.abs() > 3 {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}
