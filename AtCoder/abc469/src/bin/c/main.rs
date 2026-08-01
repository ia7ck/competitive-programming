use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let mut ans = 0;
    for _ in 0..n {
        let mut o = false;
        if ans < n {
            if s[ans] == 'o' {
                o = true;
            }
            ans += 1;
        }
        while o && ans < n {
            o = false;
            if s[ans] == 'o' {
                o = true;
            }
            ans += 1;
        }
        println!("{}", ans);
    }
}
