use ac_library::ModInt;
use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[u64; n],
    }

    let mut one = ModInt::new(n).inv();
    let mut ans = ModInt::new(0);
    let mut sum = one;
    for a in a {
        ans += sum * a;
        sum += sum * one;
    }
    println!("{}", ans.val());
}
