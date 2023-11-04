use proconio::input;

fn main() {
    input! {
        a:[[usize; 9]; 9],
    }

    if check(&a) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn check(grid: &Vec<Vec<usize>>) -> bool {
    // 1
    for i in 0..9 {
        let mut row = [false; 10];
        for j in 0..9 {
            let num = grid[i][j];
            if row[num] {
                return false;
            }
            row[num] = true;
        }
    }

    // 2
    for j in 0..9 {
        let mut col = [false; 10];
        for i in 0..9 {
            let num = grid[i][j];
            if col[num] {
                return false;
            }
            col[num] = true;
        }
    }

    // 3
    for i in 0..3 {
        for j in 0..3 {
            let mut houzin = [false; 10];
            for x in 0..3 {
                for y in 0..3 {
                    let num = grid[i * 3 + x][j * 3 + y];
                    if houzin[num] {
                        return false;
                    }
                    houzin[num] = true;
                }
            }
        }
    }

    true
}