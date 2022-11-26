use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut ans = 0;
    for ch in s {
        if ch == 'v' {
            ans += 1;
        } else {
            assert_eq!(ch, 'w');
            ans += 2;
        }
    }

    println!("{}", ans);
}
