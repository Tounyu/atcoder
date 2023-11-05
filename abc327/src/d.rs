use ac_library::Dsu;
use proconio::input;

// 2部グラフ　二部グラフ　Dsu
fn main() {
    input! {
        n:usize,
        m:usize,
        a:[usize;m],
        b:[usize;m],
    }

    let mut dsu = Dsu::new((n + 1) * 2);
    for i in 0..m {
        dsu.merge(a[i], b[i] + n);
        dsu.merge(a[i] + n, b[i]);
    }

    for i in 0..m {
        if dsu.same(a[i], b[i]) {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
