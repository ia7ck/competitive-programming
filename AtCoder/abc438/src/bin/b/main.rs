use proconio::{input, marker::Bytes};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Bytes,
        t: Bytes,
    };

    let s = s.into_iter().map(|b| b - b'0').collect::<Vec<_>>();
    let t = t.into_iter().map(|b| b - b'0').collect::<Vec<_>>();
    let mut ans = m * 10;
    for i in 0..=(n - m) {
        let c = cost(&s[i..(i + m)], &t);
        ans = ans.min(c);
    }
    println!("{}", ans);
}

fn cost(s: &[u8], t: &Vec<u8>) -> usize {
    assert_eq!(s.len(), t.len());

    let mut res = 0;
    for i in 0..s.len() {
        if s[i] >= t[i] {
            res += usize::from(s[i] - t[i]);
        } else {
            res += usize::from(10 - t[i] + s[i]);
        }
    }
    res
}
