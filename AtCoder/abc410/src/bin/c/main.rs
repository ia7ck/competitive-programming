use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    };

    let mut a = (1..=n).collect::<Vec<_>>();
    let mut rot_left = 0;
    for _ in 0..q {
        input! {
            op: u8,
        };

        if op == 1 {
            input! {
                p: Usize1,
                x: usize,
            };
            let i = (p + rot_left) % n;
            a[i] = x;
        } else if op == 2 {
            input! {
                p: Usize1,
            };
            let i = (p + rot_left) % n;
            println!("{}", a[i]);
        } else {
            input! {
                k: usize,
            };
            rot_left += k;
        }
    }
}
