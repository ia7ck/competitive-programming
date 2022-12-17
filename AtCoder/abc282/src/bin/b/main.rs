use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    };

    let mut ans = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            let ok = (0..m).all(|k| s[i][k] == 'o' || s[j][k] == 'o');
            if ok {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
