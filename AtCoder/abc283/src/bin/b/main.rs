use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u32; n],
        q: usize,
    };
    for _ in 0..q {
        input! {
            op: u8,
        };

        if op == 1 {
            input! {
                k: usize,
                x: u32,
            };
            a[k - 1] = x;
        } else {
            input! {
                k: usize,
            };

            println!("{}", a[k - 1]);
        }
    }
}
