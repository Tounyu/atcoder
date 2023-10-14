use proconio::input;

fn main() {
    input! {
        N:usize,
        A:[i64; N],
    }

    let a0 = A[0];
    for a in A{
        if a != a0{
            println!("No");
            return;
        }
    }

    println!("Yes");
}
