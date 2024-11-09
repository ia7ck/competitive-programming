use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    };

    let mut s = s;
    let mut ans = 0;
    for i in 0..=(n - k) {
        let ok = s[i..(i + k)].iter().all(|&c| c == 'O');
        if ok {
            ans += 1;
            for j in 0..k {
                s[i + j] = 'X';
            }
        }
    }

    println!("{}", ans);
}
