use proconio::{input, marker::Chars};

#[allow(unused_macros)]
macro_rules! eprintln {
    ($($arg:tt)*) => {{
        #[cfg(debug_assertions)]
        std::eprintln!($($arg)*);
    }};
}

const H: usize = 18;
const W: usize = 6;

fn accept(s: &[char], t: &[char]) -> bool {
    for i in 0..W {
        assert!(s[i] == '?' || s[i] == '0' || s[i] == '1');
        assert!(t[i] == '0' || t[i] == '1');
        if s[i] == '?' {
            continue;
        }
        if s[i] != t[i] {
            return false;
        }
    }
    true
}

fn bit_to_chars(bit: usize) -> [char; W] {
    let mut chars = ['0'; W];
    for j in 0..W {
        if bit >> j & 1 == 1 {
            chars[j] = '1';
        }
    }
    chars
}

fn main() {
    input! {
        s: [Chars; H],
    };

    let s = {
        let mut t = Vec::new();
        t.push(vec!['0'; W]);
        t.push(vec!['0'; W]);
        t.extend(s);
        t
    };

    let mut dp = vec![vec![0_u64; 1 << W]; 1 << W];
    dp[0][0] = 1;
    for i in 2..(H + 2) {
        let mut next = vec![vec![0_u64; 1 << W]; 1 << W];
        for bit_a in 0..(1 << W) {
            let a = bit_to_chars(bit_a);
            if !accept(&s[i - 2], &a) {
                continue;
            }
            for bit_b in 0..(1 << W) {
                let b = bit_to_chars(bit_b);
                if !accept(&s[i - 1], &b) {
                    continue;
                }
                for bit_c in 0..(1 << W) {
                    let c = bit_to_chars(bit_c);
                    if !accept(&s[i], &c) {
                        continue;
                    }
                    let mut ok = true;
                    for j in 0..W {
                        let mut cnt = [0, 0];
                        cnt[(a[j] == '1') as usize] += 1;
                        cnt[(j >= 1 && b[j - 1] == '1') as usize] += 1;
                        cnt[(b[j] == '1') as usize] += 1;
                        cnt[(j + 1 < W && b[j + 1] == '1') as usize] += 1;
                        cnt[(c[j] == '1') as usize] += 1;
                        if (b[j] == '0' && cnt[0] >= 3) || (b[j] == '1' && cnt[1] >= 3) {
                            // ok
                        } else {
                            ok = false;
                        }
                    }
                    if ok {
                        eprintln!("bit_a = {:06b}", bit_a);
                        eprintln!("bit_b = {:06b}", bit_b);
                        eprintln!("bit_c = {:06b}", bit_c);
                        next[bit_b][bit_c] += dp[bit_a][bit_b];
                    }
                }
            }
        }
        dp = next;
    }

    let mut ans = 0;
    for bit_a in 0..(1 << W) {
        let a = bit_to_chars(bit_a);
        if !accept(&s[H], &a) {
            continue;
        }
        for bit_b in 0..(1 << W) {
            eprintln!("bit_a = {:06b}", bit_a);
            eprintln!("bit_b = {:06b}", bit_b);
            eprintln!("dp    = {}", dp[bit_a][bit_b]);
            let b = bit_to_chars(bit_b);
            if !accept(&s[H + 1], &b) {
                continue;
            }
            let mut ok = true;
            for j in 0..W {
                let mut cnt = [1, 0];
                cnt[(a[j] == '1') as usize] += 1;
                cnt[(j >= 1 && b[j - 1] == '1') as usize] += 1;
                cnt[(b[j] == '1') as usize] += 1;
                cnt[(j + 1 < W && b[j + 1] == '1') as usize] += 1;
                if (b[j] == '0' && cnt[0] >= 3) || (b[j] == '1' && cnt[1] >= 3) {
                    // ok
                } else {
                    ok = false;
                }
            }
            if ok {
                ans += dp[bit_a][bit_b];
            }
        }
    }
    println!("{}", ans);
}
