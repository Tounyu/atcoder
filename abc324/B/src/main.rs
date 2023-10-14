use proconio::input;

fn main() {
    input! {
        N:u64,
    }

    for x in 0..60 {
        let X = 2u64.pow(x);
        for y in 0..32 {
            let Y = 3u64.pow(y);

            if X * Y > 10u64.pow(18){
                break;
            }

            if N == X * Y{
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
