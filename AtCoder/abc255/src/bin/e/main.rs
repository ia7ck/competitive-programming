use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [i64; n - 1],
        x: [i64; m],
    };

    // a[0] = p
    // a[1] = s[0] - p
    // a[2] = s[1] - a[1] = s[1] - (s[0] - p)
    // a[3] = s[2] - a[2] = s[2] - (s[1] - (s[0] - p))

    let mut b = vec![0];
    for i in 0..(n - 1) {
        let last = b[b.len() - 1];
        b.push(s[i] - last);
    }
    let mut f_even = HashMap::new();
    let mut f_odd = HashMap::new();
    for i in 0..n {
        if i % 2 == 0 {
            *f_even.entry(b[i]).or_insert(0) += 1;
        } else {
            *f_odd.entry(b[i]).or_insert(0) += 1;
        }
    }
    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            // a[i] = x[j]
            let p = if i % 2 == 0 { x[j] - b[i] } else { b[i] - x[j] };
            let mut tmp = 0;
            for k in 0..m {
                if let Some(f) = f_even.get(&(x[k] - p)) {
                    tmp += f;
                }
                if let Some(f) = f_odd.get(&(x[k] + p)) {
                    tmp += f;
                }
            }
            ans = ans.max(tmp);
        }
    }
    println!("{}", ans);
}
