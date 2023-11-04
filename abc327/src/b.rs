use proconio::input;

fn main() {
    input! {
        b:u128,
    }

    for i in 1u128..=16 {
        let a = i.pow(i as u32);
        if a == b {
            println!("{}", i);
            return;
        }
    }
    println!("-1");
}
