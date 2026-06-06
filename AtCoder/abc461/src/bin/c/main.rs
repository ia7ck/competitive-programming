use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        m: usize,
        cv: [(Usize1, u64); n],
    };

    let mut v_by_c = vec![vec![]; n];
    for (c, v) in cv {
        v_by_c[c].push(v);
    }
    for c in 0..n {
        v_by_c[c].sort_unstable();
    }

    let mut ord = (0..n).collect::<Vec<_>>();
    ord.sort_unstable_by_key(|&i| v_by_c[i].last().copied());
    ord.reverse();
    let mut ans = 0;
    for &i in ord.iter().take(m) {
        let last = v_by_c[i].pop().unwrap();
        ans += last;
    }

    let mut rest = Vec::new();
    for v in v_by_c {
        rest.extend(v);
    }
    rest.sort_unstable();
    rest.reverse();
    for v in rest.iter().take(k - m) {
        ans += v;
    }

    println!("{}", ans);
}
