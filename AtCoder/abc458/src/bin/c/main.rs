use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut ans = 0_usize;
    for i in 0..s.len() {
        if s[i] == 'C' {
            let l = i;
            let r = s.len() - i - 1;
            ans += 1 + l.min(r);
        }
    }
    println!("{}", ans);
}
