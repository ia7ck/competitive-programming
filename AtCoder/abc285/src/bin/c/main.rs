use proconio::{input, marker::Bytes};

fn main() {
    input! {
        s: Bytes,
    };

    let mut cnt = 0;
    for i in 1..s.len() {
        cnt += 26_usize.pow(i as u32);
    }
    dbg!(cnt);
    let mut k = 0;
    for b in s {
        k = k * 26 + (b - b'A') as usize;
    }
    let ans = cnt + 1 + k;
    println!("{}", ans);
}
