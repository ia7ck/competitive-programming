use proconio::{input, marker::Bytes};

fn check(c: u8, d: u8) -> bool {
    c == d || c == b'?' || d == b'?'
}

fn main() {
    input! {
        s: Bytes,
        t: Bytes,
    };

    let n = s.len();
    let m = t.len();
    let mut accept = 0;
    for i in 0..m {
        if check(s[i + n - m], t[i]) {
            accept += 1;
        }
    }
    let mut ans = Vec::new();
    ans.push(accept == m);
    for i in 0..m {
        if check(s[i + n - m], t[i]) {
            assert!(accept >= 1);
            accept -= 1;
        }
        if check(s[i], t[i]) {
            accept += 1;
        }
        ans.push(accept == m);
    }

    for ans in ans {
        if ans {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
