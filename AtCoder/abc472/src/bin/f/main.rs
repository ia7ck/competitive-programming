use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        xy: [(i64, i64); n],
        queries: [(Usize1, Usize1); q],
    };

    let cross = |i: usize, j: usize| -> i64 {
        let (xi, yi) = xy[i];
        let (xj, yj) = xy[j];
        xi * yj - xj * yi
    };
    let x_cross = |i: usize, j: usize| -> i64 {
        let (xi, _) = xy[i];
        let (xj, _) = xy[j];
        (xi + xj) * cross(i, j)
    };
    let y_cross = |i: usize, j: usize| -> i64 {
        let (_, yi) = xy[i];
        let (_, yj) = xy[j];
        (yi + yj) * cross(i, j)
    };

    let mut c = Vec::new();
    let mut cx = Vec::new();
    let mut cy = Vec::new();
    for i in 0..n {
        c.push(cross(i, (i + 1) % n));
        cx.push(x_cross(i, (i + 1) % n));
        cy.push(y_cross(i, (i + 1) % n));
    }
    let mut cc = c.clone();
    let mut ccx = cx.clone();
    let mut ccy = cy.clone();
    for i in 1..n {
        cc[i] += cc[i - 1];
        ccx[i] += ccx[i - 1];
        ccy[i] += ccy[i - 1];
    }

    let query = |u: usize, v: usize| -> (i64, i64, i64) {
        assert!(u < v);

        let mut a = cc[v - 1];
        if u >= 1 {
            a -= cc[u - 1];
        }
        a += cross(v, u);

        let mut x = ccx[v - 1];
        if u >= 1 {
            x -= ccx[u - 1];
        }
        x += x_cross(v, u);

        let mut y = ccy[v - 1];
        if u >= 1 {
            y -= ccy[u - 1];
        }
        y += y_cross(v, u);

        (a, x, y)
    };

    for (u, v) in queries {
        let (a, x, y) = if u < v {
            query(u, v)
        } else {
            let (a, x, y) = query(v, u);
            let (a0, x0, y0) = query(0, n - 1);
            (a0 - a, x0 - x, y0 - y)
        };

        let den = 6.0 * (a as f64 / 2.0);

        println!("{} {}", x as f64 / den, y as f64 / den);
    }
}
