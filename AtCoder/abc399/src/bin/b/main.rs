use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [u32; n],
    };

    let r = {
        let mut r = HashMap::new();
        let mut p = p.clone();
        let mut rank = 1;
        while !p.is_empty() {
            let m = p.iter().max().copied().unwrap();
            r.insert(m, rank);
            let k = p.iter().filter(|&&x| x == m).count();
            rank += k;
            p.retain(|&x| x != m);
        }
        r
    };

    for p in p {
        let ans = r[&p];
        println!("{}", ans);
    }
}
