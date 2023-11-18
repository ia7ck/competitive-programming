use proconio::{input, marker::Bytes};

fn main() {
    input! {
        n: usize,
        s: Bytes,
    };

    let (mut c, mut l) = (b'a', 0_usize);
    let mut max_len = vec![0; 26];
    for i in 0..n {
        if c == s[i] {
            l += 1;
        } else {
            let j = usize::from(c - b'a');
            max_len[j] = max_len[j].max(l);
            c = s[i];
            l = 1;
        }
    }
    let j = usize::from(c - b'a');
    max_len[j] = max_len[j].max(l);
    let mut ans = 0;
    for l in max_len {
        ans += l;
    }
    println!("{}", ans);
}
