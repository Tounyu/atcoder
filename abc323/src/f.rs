use proconio::input;

fn main() {
    input! {
        mut xa:i64,
        mut ya:i64,
        xb:i64,
        yb:i64,
        mut xc:i64,
        mut yc:i64,
    }

    // 荷物を原点へ補正
    xa -= xb;
    ya -= yb;
    xc -= xb;
    yc -= yb;

    let mut before = 0;
    if ya == 0 && yc == 0 {
        if (xa < 0 && xc < 0) || (0 < xa && 0 < xc) {
            before += 4;
        }
    }
    if xa == 0 && xc == 0 {
        if (ya < 0 && yc < 0) || (0 < ya && 0 < yc) {
            before += 4;
        }
    }
    if 0 < xa && 0 < ya && 0 < xc && 0 < yc {
        before += xa.abs() + ya.abs() - 1 + 2;
    } else if 0 < xa && 0 > ya && 0 < xc && 0 > yc {
        before += xa.abs() + ya.abs() - 1 + 2;
    } else if 0 > xa && 0 < ya && 0 > xc && 0 < yc {
        before += xa.abs() + ya.abs() - 1 + 2;
    } else if 0 > xa && 0 > ya && 0 > xc && 0 > yc {
        before += xa.abs() + ya.abs() - 1 + 2;
    } else {
        before += xa.abs() + ya.abs() - 1;
    }

    // 最適な場所から荷物を押し始める
    let after = xc.abs() + yc.abs() + rotate(0, xc, 0, yc); // 荷物の移動

    println!("{}", before + after);
}

fn rotate(from_x:i64,to_x:i64,from_y:i64,to_y:i64) -> i64 {
    if from_x == to_x || from_y == to_y {
        return 0;
    }
    return 2;
}
