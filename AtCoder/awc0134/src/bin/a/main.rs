use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        cl: [(Usize1, u64); n],
    };

    let mut ls_by_c = vec![vec![]; n];
    for (c, l) in cl {
        ls_by_c[c].push(l);
    }

    let mut ans = 0;
    for ls in ls_by_c {
        for i in 0..ls.len() {
            for j in (i + 1)..ls.len() {
                ans += ls[i].abs_diff(ls[j]);
            }
        }
    }

    println!("{}", ans);
}
