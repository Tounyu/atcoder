use proconio::input;

fn main() {
    input! {
        n: i64,
    }

    for i in n..=919{
        let c :Vec<u8>= i.to_string().chars().map(|a| a.to_digit(10).unwrap() as u8).collect();
        if c[0] * c[1] == c[2] {
            println!("{}", i);
            return;
        }
    }
}
