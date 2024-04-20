use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        t: [Usize1; q],
    };

    let mut exist = vec![true; n];
    for t in t {
        exist[t] = !exist[t];
    }
    let ans = exist.iter().filter(|&&f| f).count();
    println!("{}", ans);
}
