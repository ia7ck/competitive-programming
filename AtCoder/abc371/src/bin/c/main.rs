use next_permutation::NextPermutation;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m_g: usize,
        edges_g: [(Usize1, Usize1); m_g],
        m_h: usize,
        edges_h: [(Usize1, Usize1); m_h],
    };

    let mut c = vec![vec![]; n];
    for i in 0..(n - 1) {
        input! {
            a_: [u64; n - i - 1],
        };
        c[i] = a_;
    }

    let mut ans = u64::MAX;
    let mut p = (0..n).collect::<Vec<_>>();
    loop {
        let mut cost = 0;
        for i in 0..n {
            for j in (i + 1)..n {
                let (pi, pj) = (p[i], p[j]);
                let (pi, pj) = (pi.min(pj), pi.max(pj));
                if edges_g.contains(&(i, j)) {
                    if !edges_h.contains(&(pi, pj)) {
                        cost += c[pi][pj - pi - 1];
                    }
                } else {
                    if edges_h.contains(&(pi, pj)) {
                        cost += c[pi][pj - pi - 1];
                    }
                }
            }
        }

        ans = ans.min(cost);

        if !p.next_permutation() {
            break;
        }
    }

    println!("{}", ans);
}
