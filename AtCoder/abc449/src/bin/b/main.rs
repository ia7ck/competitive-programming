use proconio::input;

fn main() {
    input! {
        mut h: usize,
        mut w: usize,
        q: usize,
    };

    for _ in 0..q {
        input! {
            op: u8,
        };

        if op == 1 {
            input! {
                r: usize,
            };

            let ans = r * w;
            println!("{}", ans);

            h -= r;
        } else {
            input! {
                c: usize,
            };

            let ans = c * h;
            println!("{}", ans);

            w -= c;
        }
    }
}
