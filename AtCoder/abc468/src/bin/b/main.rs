use proconio::{input, marker::Chars};

fn main() {
    input! {
        m: usize,
        d: usize,
        s: Chars,
    };

    let mut watch = vec![false; m];
    for i in 0..m {
        if s[i] == 'G' {
            for j in i.saturating_sub(d)..=(i + d).min(m - 1) {
                watch[j] = true;
            }
        }
    }

    let ans = watch.into_iter().filter(|&w| w == false).count();
    println!("{}", ans);
}
