use proconio::input;

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
            h: i64,
            queries: [(i64, i64, i64); n],
        };

        solve(n, h, queries);
    }
}

fn solve(_n: usize, h: i64, queries: Vec<(i64, i64, i64)>) {
    let mut t = 0;
    let (mut l, mut u) = (h, h);
    for (tt, ll, uu) in queries {
        let dt = tt - t;
        let new_l = (l - dt).max(1);
        let new_u = u + dt;
        if new_l > uu || new_u < ll {
            println!("No");
            return;
        }
        t = tt;
        (l, u) = (new_l.max(ll), new_u.min(uu));
    }

    println!("Yes");
}
