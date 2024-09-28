use proconio::{input, marker::Bytes};

fn main() {
    input! {
        s: Bytes,
    };

    let mut pos = vec![0; 26];
    for (i, &ch) in s.iter().enumerate() {
        pos[usize::from(ch) - usize::from(b'A')] = i;
    }

    let mut p = pos[0]; // 'A'
    let mut ans = 0;
    for &ch in b"BCDEFGHIJKLMNOPQRSTUVWXYZ" {
        let new_p = pos[usize::from(ch) - usize::from(b'A')];
        ans += p.abs_diff(new_p);
        p = new_p;
    }
    println!("{}", ans);
}
