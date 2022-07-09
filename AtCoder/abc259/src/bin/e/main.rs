use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
    };

    let mut pes = Vec::new();
    for _ in 0..n {
        input! {
            m: usize,
            pe: [(u32, u32); m],
        };
        pes.push(pe);
    }

    let mut max_e = HashMap::new();
    for pe in &pes {
        for &(p, e) in pe {
            let mx = max_e.entry(p).or_insert(0);
            *mx = (*mx).max(e);
        }
    }

    let mut max_e_count = HashMap::new();
    for pe in &pes {
        for &(p, e) in pe {
            if max_e[&p] == e {
                *max_e_count.entry(p).or_insert(0) += 1;
            }
        }
    }

    let mut ans = 1;
    for pe in &pes {
        let mut ok = false;
        for &(p, e) in pe {
            if max_e[&p] == e && max_e_count[&p] == 1 {
                ok = true;
            }
        }
        if ok {
            ans += 1;
        }
    }
    println!("{}", ans.min(n)); // ?
}
