use proconio::input;

fn main() {
    input! {
        q: usize,
    };

    let mut a = Vec::new();
    for _ in 0..q {
        input! {
            op: u8,
        };
        if op == 1 {
            input! {
                x: u64,
            };
            a.push(x);
        } else {
            input! {
                k: usize,
            };
            assert!(k <= a.len());
            let ans = a[a.len() - k];
            println!("{}", ans);
        }
    }
}
