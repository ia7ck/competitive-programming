use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        t: Chars,
    };

    let mut s = s;
    let mut i = 0;
    let mut r = 0;
    let mut end = false;
    let mut used = vec![false; m];
    for c in ('1'..='9').rev() {
        let positions = (0..m).filter(|&i| t[i] == c).collect::<Vec<_>>();
        if positions.is_empty() {
            continue;
        }
        for &p in &positions {
            assert!(!used[p]);
            while i < n {
                if s[i] < c {
                    s[i] = c;
                    i += 1;
                    if i == n {
                        end = true;
                    }
                    used[p] = true;
                    break;
                } else {
                    i += 1;
                }
            }
        }
        let j = positions.iter().max().copied().unwrap();
        if r < j {
            if end {
                s[n - 1] = c;
            }
            r = j;
        }
    }

    if !used[m - 1] {
        if !s.contains(&t[m - 1]) {
            s[n - 1] = t[m - 1];
        }
    }

    println!("{}", s.iter().collect::<String>());
}
