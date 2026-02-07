use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
    };

    a.sort_unstable();

    let mut ans = Vec::new();
    for l in [a[0] + a[n - 1], a[n - 1]] {
        if f(&a, l) {
            ans.push(l);
        }
    }
    ans.sort_unstable();
    ans.dedup();

    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

fn f(a: &Vec<u64>, l: u64) -> bool {
    use std::collections::hash_map::Entry;

    let mut counter = HashMap::new();
    for &a in a {
        *counter.entry(a).or_insert(0) += 1;
    }

    loop {
        let Some(a) = counter.keys().next().copied() else {
            break;
        };

        assert!(a <= l);
        if a == l {
            counter.remove(&a);
            continue;
        }

        for key in [a, l - a] {
            let Entry::Occupied(o) = counter.entry(key) else {
                return false;
            };
            let e = o.into_mut();
            *e -= 1;
            if *e == 0 {
                counter.remove(&key);
            }
        }
    }

    true
}
