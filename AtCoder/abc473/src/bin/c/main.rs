use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [Usize1; n],
    };

    let mut class = vec![0; k];
    for a in a {
        class[a] += 1;
    }
    let cmax = class.iter().max().copied().unwrap();
    let mut ans = 0;
    for c in class {
        if c == cmax || c == cmax - 1 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
