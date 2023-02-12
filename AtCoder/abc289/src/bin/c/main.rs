use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: u8,
        m: usize,
        a: [[u8]; m],
    };
    
    let mut ans = 0;
    for bits in 0..(1 << m) {
        let mut b = HashSet::new();
        for i in 0..m {
            if bits >> i & 1 == 1 {
                for &y in &a[i] {
                    b.insert(y);
                }
            }
        }
        let mut ok = true;
        for x in 1..=n {
            if b.contains(&x) == false {
                ok = false;
            }
        }
        if ok {
            ans += 1;
        }
    }

    println!("{}", ans);
}
