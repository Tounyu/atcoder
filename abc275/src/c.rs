use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s:[Chars; 9],
    }

    let mut count = 0;

    for ai in 0..9 {
        for aj in 0..9 {
            for bi in 0..9 {
                for bj in 0..9 {
                    if ai == bi && aj == bj { continue; }
                        count += valid(ai,aj,bi,bj,&s);
                }
            }
        }
    }

    println!("{}", count / 8);
}

fn valid(ai:i32, aj:i32, bi:i32, bj:i32, s:&Vec<Vec<char>>) -> i32 {
    if !ok(ai, aj) { return 0; }
    if !ok(bi, bj) { return 0; }

    if s[ai as usize][aj as usize] != '#' { return 0; }
    if s[bi as usize][bj as usize] != '#' { return 0; }

    let diff_i = aj - bj;
    let diff_j = ai - bi;

    let mut count =0;

    for i in [-1, 1] {
        let ci = ai - i * diff_i;
        let cj = aj + i * diff_j;
        let di = bi - i * diff_i;
        let dj = bj + i * diff_j;

        if !ok(ci,cj) { continue; }
        if !ok(di,dj) { continue;}

        if s[ci as usize][cj as usize] != '#' { continue; }
        if s[di as usize][dj as usize] != '#' { continue; }

        // println!("({ai},{aj})({bi},{bj})({ci},{cj})({di},{dj})");

        count += 1;
    }

    return count;
}

fn ok(a:i32, b:i32) -> bool {
    return 0 <= a && a <= 8 && 0 <= b && b <= 8;
}
