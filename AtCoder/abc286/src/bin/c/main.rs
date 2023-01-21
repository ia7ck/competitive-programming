use proconio::{input, marker::Bytes};

fn main() {
    input! {
        n: usize,
        a: u64,
        b: u64,
        s: Bytes,
    };

    let calc = |t: &[u8]| -> u64 {
        assert_eq!(t.len(), n);
        let mut result = 0;
        for i in 0..(n / 2) {
            if t[i] != t[n - i - 1] {
                result += b;
            }
        }
        result
    };

    let mut t = s.clone();
    t.extend(s.clone());
    let mut ans = b * (n / 2) as u64;
    for i in 0..n {
        let t = &t[i..(i + n)];
        ans = ans.min(a * i as u64 + calc(t));
    }
    println!("{}", ans);
}
