use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [Usize1; n],
        b: [Usize1; n],
        queries: [(Usize1, Usize1, Usize1, Usize1); q],
    };

    let mut h = vec![0; n];
    for i in 0..n {
        let mut state = DefaultHasher::new();
        i.hash(&mut state);
        h[i] = state.finish();
    }

    const M: u64 = 998_244_353;
    let mut cumsum_a = vec![0; n + 1];
    let mut cumsum_b = vec![0; n + 1];
    for i in 0..n {
        cumsum_a[i + 1] = (cumsum_a[i] + h[a[i]]) % M;
        cumsum_b[i + 1] = (cumsum_b[i] + h[b[i]]) % M;
    }

    for (l, r, ll, rr) in queries {
        let h_a = (M + cumsum_a[r + 1] - cumsum_a[l]) % M;
        let h_b = (M + cumsum_b[rr + 1] - cumsum_b[ll]) % M;
        if h_a == h_b {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
