use proconio::input;

fn main() {
    input! {
        t:usize,
        a:[(usize,usize);t],
    }
    for (a, s) in a {
        if 2 * a <= s && (s-2 *a) & a == 0 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
