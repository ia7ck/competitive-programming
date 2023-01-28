use proconio::{input, marker::Bytes};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Bytes; n],
        t: [Bytes; m],
    };

    let mut ans = 0;
    for s in s {
        if t.iter().any(|t| &s[3..] == &t[..]) {
            ans += 1;
        }
    }
    println!("{}", ans);
}
