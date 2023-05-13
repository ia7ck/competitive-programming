use proconio::{input, marker::Bytes};

fn main() {
    input! {
        s: Bytes,
        t: Bytes,
    };

    let (mut at_s, mut at_t) = (0, 0);
    let (mut f, mut g) = ([0; 26], [0; 26]);
    for &b in &s {
        if b == b'@' {
            at_s += 1;
        } else {
            f[(b - b'a') as usize] += 1;
        }
    }
    for &b in &t {
        if b == b'@' {
            at_t += 1;
        } else {
            g[(b - b'a') as usize] += 1;
        }
    }

    let atcoder = |i: usize| -> bool {
        let b = i as u8 + b'a';
        "atcoder".bytes().any(|bb| bb == b)
    };

    let mut ok = true;
    for i in 0..26 {
        if f[i] < g[i] {
            if atcoder(i) == false || at_s < g[i] - f[i] {
                ok = false;
                break;
            }
            at_s -= g[i] - f[i];
        } else if f[i] > g[i] {
            if  atcoder(i) == false || at_t < f[i] - g[i] {
                ok = false;
                break;
            }
            at_t -= f[i] - g[i];
        }
    }

    if ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
