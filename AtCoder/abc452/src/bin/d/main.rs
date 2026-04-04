use proconio::{input, marker::Bytes};

fn main() {
    input! {
        s: Bytes,
        t: Bytes,
    };

    // s = 'a'
    // t = 'a'

    let mut next = vec![vec![usize::MAX; 26]; s.len()];
    for i in (0..s.len() - 1).rev() {
        for j in 0..26 {
            next[i][j] = next[i + 1][j];
        }
        next[i][usize::from(s[i + 1] - b'a')] = i + 1;
    }

    // [l, r]
    let contains = |l: usize, r: usize| -> bool {
        if l > r {
            return false;
        }
        let mut p = l;
        for (i, &t) in t.iter().enumerate() {
            if i == 0 && s[l] == t {
                p = l;
            } else {
                p = next[p][usize::from(t - b'a')];
            };
            if p > r {
                return false;
            }
        }
        true
    };

    let mut sub = 0_usize;
    let mut l = 0;
    for r in 0..s.len() {
        while contains(l, r) {
            l += 1;
        }
        // [0, r], [1, r], ..., [l-1, r]
        sub += l;
    }

    // 1 + 2 + ... + n
    let total = s.len() * (s.len() + 1) / 2;
    let ans = total - sub;

    println!("{}", ans);
}
