use proconio::input;

fn main() {
    input! {
        n: usize,
        d: u32,
        t: [u32; n],
    };

    for w in t.windows(2) {
        if w[1] - w[0] <= d {
            println!("{}", w[1]);
            return;
        }
    }

    println!("-1");
}
