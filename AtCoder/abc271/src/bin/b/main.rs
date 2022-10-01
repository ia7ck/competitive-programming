use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    };
    let mut a = vec![vec![]; n];
    for i in 0..n {
        input! {
            l: usize,
            b: [u32; l],
        };
        a[i] = b;
    }
    for _ in 0..q {
        input! {
            s: usize,
            t: usize,
        };
        println!("{}", a[s - 1][t - 1]);
    }
}
