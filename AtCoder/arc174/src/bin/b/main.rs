use proconio::input;

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            a: [i64; 5],
            p: [i64; 5],
        };

        solve(a, p);
    }
}

fn solve(a: Vec<i64>, p: Vec<i64>) {
    let (a1, a2, a3, a4, a5) = (a[0], a[1], a[2], a[3], a[4]);
    let (_, _, _, p4, p5) = (p[0], p[1], p[2], p[3], p[4]);

    // (a1 * 1 + a2 * 2 + a3 * 3 + (a4 + x) * 4 + (a5 + y) * 5) / (a1 + a2 + a3 + (a4 + x) + (a5 + y)) >= 3
    // x + y * 2 ≧ (-1) * (a1 * (-2) + a2 * (-1) + a3 * 0 + a4 * 1 + a5 * 2)

    // minimize p4 * x + p5 * y

    let r = (-1) * (a1 * (-2) + a2 * (-1) + a3 * 0 + a4 * 1 + a5 * 2);
    if r <= 0 {
        println!("0");
        return;
    }

    let mut ans = (r / 2) * ((p4 * 2).min(p5));
    if r % 2 == 1 {
        ans += p4.min(p5);
    }
    println!("{}", ans);
}
