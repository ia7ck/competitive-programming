use std::collections::{BTreeSet, HashMap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        queries: [(usize, usize); q],
    };

    let mut values = Vec::new();
    for &(l, r) in &queries {
        values.push(l);
        values.push(r);
    }
    values.sort_unstable();
    values.dedup();

    let mut a = Vec::new();
    let mut a_index = HashMap::new();

    // 1, 2, ..., values[0]-1
    a.push(values[0] - 1);

    a_index.insert(values[0], a.len());
    a.push(1);
    for w in values.windows(2) {
        // w[0]+1, w[0]+2, ..., w[1]-1
        a.push(w[1] - w[0] - 1);

        a_index.insert(w[1], a.len());
        a.push(1);
    }

    // last+1, last+2, ..., n
    a.push(n - values.last().unwrap());

    assert_eq!(a.iter().sum::<usize>(), n);

    let mut white = (0..a.len()).collect::<BTreeSet<_>>();
    let mut ans = n;
    for (l, r) in queries {
        let l = a_index[&l];
        let r = a_index[&r];
        let del = white.range(l..=r).cloned().collect::<Vec<_>>();
        for i in del {
            white.remove(&i);
            ans -= a[i];
        }
        println!("{}", ans);
    }
}
