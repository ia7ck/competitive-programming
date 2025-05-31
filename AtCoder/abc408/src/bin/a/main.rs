use proconio::input;

fn main() {
    input! {
        n: usize,
        s: u32,
        t: [u32; n],
    };

    let mut last = 0;
    for t in t {
        // last + s + 0.5 <= t
        if last + s < t {
            println!("No");
            return;
        }
        last = t;
    }

    println!("Yes");
}
