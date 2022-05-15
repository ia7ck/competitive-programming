use proconio::input;
use proconio::marker::Chars;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let mut b = Vec::new();
    for i in 1..(n - 1) {
        if s[(i - 1)..=(i + 1)] == ['A', 'R', 'C'] {
            let mut l = i;
            while l >= 1 && s[l - 1] == 'A' {
                l -= 1;
            }
            let mut r = i;
            while r + 1 < n && s[r + 1] == 'C' {
                r += 1;
            }
            // eprintln!("{:?}", &s[l..=r]);
            // eprintln!("a = {}, c = {}", i - l, r - i);
            b.push((i - l).min(r - i));
        }
    }
    let mut set = BTreeSet::new();
    for i in 0..b.len() {
        set.insert((b[i], i));
    }
    let mut ans = 0;
    while set.len() >= 1 {
        ans += 1;
        if ans % 2 == 1 {
            let (x, i) = set.iter().last().copied().unwrap();
            set.remove(&(x, i));
            if x >= 2 {
                set.insert((x - 1, i));
            }
        } else {
            let (x, i) = set.iter().next().copied().unwrap();
            set.remove(&(x, i));
        }
    }
    println!("{}", ans);
}
