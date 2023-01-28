use proconio::{input, marker::Bytes};

fn calc(s: &[u8], t: &[u8]) -> usize {
    let n = s.len().min(t.len());
    for i in 0..n {
        if s[i] != t[i] {
            return i;
        }
    }
    n
}

fn main() {
    input! {
        n: usize,
        s: [Bytes; n],
    };

    let mut ord = Vec::new();
    for i in 0..n {
        ord.push(i);
    }
    ord.sort_by(|&i, &j| s[i].cmp(&s[j]));
    let mut ans = vec![0; n];
    for k in 0..n {
        if k >= 1 {
            ans[ord[k]] = ans[ord[k]].max(calc(&s[ord[k - 1]], &s[ord[k]]));
        }
        if k + 1 < n {
            ans[ord[k]] = ans[ord[k]].max(calc(&s[ord[k]], &s[ord[k + 1]]));
        }
    }
    for ans in ans {
        println!("{}", ans);
    }
}
