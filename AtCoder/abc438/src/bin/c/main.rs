use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    };

    let mut b = Vec::new();
    for a in a {
        b.push(a);
        let m = b.len();
        if m >= 4 && b[m - 4] == b[m - 3] && b[m - 3] == b[m - 2] && b[m - 2] == b[m - 1] {
            b.pop();
            b.pop();
            b.pop();
            b.pop();
        }
    }

    println!("{}", b.len());
}
