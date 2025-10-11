use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            k: usize,
            s: Chars,
            edges: [(Usize1, Usize1); m],
        };

        solve(n, m, k, s, edges);
    }
}

fn solve(n: usize, _m: usize, k: usize, s: Vec<char>, edges: Vec<(usize, usize)>) {
    let mut a_win = vec![vec![false; n]; k * 2 + 1];
    for i in 0..n {
        if s[i] == 'A' {
            a_win[0][i] = true;
        }
    }

    for k in 1..=(k * 2) {
        if k % 2 == 1 {
            a_win[k].fill(true);
        }
        for &(u, v) in &edges {
            // Bob
            if k % 2 == 1 {
                a_win[k][u] &= a_win[k - 1][v];
            } else {
                a_win[k][u] |= a_win[k - 1][v];
            }
        }
    }

    if a_win[k * 2][0] {
        println!("Alice");
    } else {
        println!("Bob");
    }
}
