use proconio::input;
use union_find::UnionFind;

fn main() {
    input! {
        n: usize,
        sx: i64,
        sy: i64,
        tx: i64,
        ty: i64,
        xyr: [(i64, i64, i64); n],
    };

    let f = |i, j| {
        let (x, y, r) = xyr[i];
        let (xx, yy, rr) = xyr[j];
        let d = (x - xx) * (x - xx) + (y - yy) * (y - yy);
        (r - rr) * (r - rr) <= d && d <= (r + rr) * (r + rr)
    };

    let mut uf = UnionFind::new(n);
    for i in 0..n {
        for j in (i + 1)..n {
            if f(i, j) {
                uf.unite(i, j);
            }
        }
    }

    let g = |i, j| {
        let (x, y, r) = xyr[i];
        let (xx, yy, rr) = xyr[j];
        (sx - x) * (sx - x) + (sy - y) * (sy - y) == r * r
            && (tx - xx) * (tx - xx) + (ty - yy) * (ty - yy) == rr * rr
    };

    for i in 0..n {
        for j in 0..n {
            if g(i, j) {
                if uf.same(i, j) {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}
