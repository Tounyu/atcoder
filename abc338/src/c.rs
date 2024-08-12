#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        n:usize,
        Q:[usize;n],
        A:[usize;n],
        B:[usize;n],
    }

    let mut amax = 0;
    let mut bmax = 0;
    for i in 0..n {
        if A[i] > 0 {
            amax = amax.max(Q[i] / A[i]);
        }
        if B[i] > 0 {
            bmax = bmax.max(Q[i] / B[i]);
        }
    }
    let mut r = amax + bmax;
    let mut l = 0;
    let mut mid = l + (r - l) / 2;

    while l + 1 < r {
        // println!("l={l}, r={r}");
        if check(&Q, &A, &B, n, mid) {
            // println!("true");
            l = mid;
        } else {
            // println!("false");
            r = mid;
        }
        mid = l + (r - l) / 2;
    }

    if check(&Q, &A, &B, n, mid + 1) {
        println!("{}", mid + 1);
        return;
    }
    println!("{mid}");
}

fn check(Q:&Vec<usize>, A:&Vec<usize>, B:&Vec<usize>, N:usize, n:usize) -> bool {
    for i in  0..=n {
        let a = i;
        let b = n-i;
        let mut ok = true;
        for j in 0..N {
            if A[j] * a + B[j] * b > Q[j] {
                ok = false;
            }
        }
        if ok {
            return true;
        }
    }

    return false;
}