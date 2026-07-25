use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut odd = 0_usize;
    for i in 0..s.len() {
        odd += 1;
        let mut diff = 0;
        for d in 1.. {
            // l < 0 || r >= n
            if i < d || i + d >= s.len() {
                break;
            }
            let l = i - d;
            let r = i + d;
            if s[l] == s[r] {
                odd += 1;
            } else {
                if diff == 0 {
                    odd += 1;
                    diff += 1;
                } else {
                    assert_eq!(diff, 1);
                    break;
                }
            }
        }
    }

    let mut even = 0_usize;
    for i in 1..s.len() {
        even += 1;
        let mut l = i - 1;
        let mut r = i;
        let mut diff = usize::from(s[l] != s[r]);
        while l > 0 && r + 1 < s.len() {
            l -= 1;
            r += 1;
            if s[l] == s[r] {
                even += 1;
            } else {
                if diff == 0 {
                    even += 1;
                    diff += 1;
                } else {
                    assert_eq!(diff, 1);
                    break;
                }
            }
        }
    }

    dbg!(odd, even);
    println!("{}", odd + even);
}
