use proconio::input;

// 全探索 bit全探索
fn main() {
    input! {
        n:usize,
    }
    let mut set = vec![];
    for _i in 0..n {
        input! {
            a:usize,
            xy:[(usize,usize);a],
        }
        set.push(xy);
    }

    let mut ans = 0;
    for i in 0..(1 << n) {

        let mut count = 0;
        let mut honest = vec![false; n];

        for j in 0..n {
            if i & (1 << j) != 0 {
                count += 1;
                honest[j] = true;
            }
        }

        let mut valid = true;
        for j in 0..n {
            if !honest[j] { continue; }
            for &(x,y) in &set[j] {
                if (y == 1 && !honest[x - 1]) || (y == 0 && honest[x - 1]) {
                    valid = false;
                    break;
                }
            }
        }

        if valid {
            ans = ans.max(count);
        }
    }
    println!("{ans}");
}
