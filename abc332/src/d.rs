#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        h:usize,
        w:usize,
        a:[[usize; w]; h],
        b:[[usize; w]; h],
    }

    let mut ans = usize::MAX;

    // 行の入れ替えの全探索
    for mut perm_rows in (0..h).permutations(h) {
        // 列の入れ替えの全探索
        for mut perm_cols in (0..w).permutations(w) {
            let mut newA = vec![vec![0usize; w]; h];
            for (ni, &ai) in perm_rows.iter().enumerate() {
                for (nj , &aj) in perm_cols.iter().enumerate() {
                    newA[ni][nj] = a[ai][aj];
                    if b == newA {
                        let hcount = merge_sort_and_count_inversions(&mut perm_rows);
                        let wcount = merge_sort_and_count_inversions(&mut perm_cols);
                        println!("{}", hcount + wcount);
                        return;
                    }
                }
            }
        }
    }

    println!("-1");
}

fn merge_sort_and_count_inversions(arr: &mut Vec<usize>) -> usize {
    let n = arr.len();
    if n <= 1 {
        return 0;
    }

    let mid = n / 2;
    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    let mut inversions = merge_sort_and_count_inversions(&mut left);
    inversions += merge_sort_and_count_inversions(&mut right);

    let (mut i, mut j, mut k) = (0, 0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
            inversions += (left.len() - i) as usize;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }

    inversions
}