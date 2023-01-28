use proconio::{input, marker::Usize1};

const M: u64 = 998244353;

macro_rules! add {
    ($a: expr, $b: expr) => {
        $a = ($a + $b) % M;
    };
}

fn solve(i: usize, p: usize, g: &Vec<Vec<usize>>) -> (Vec<u64>, Vec<u64>) {
    let mut incl = vec![0, 1];
    let mut excl = vec![1, 0];
    for &j in &g[i] {
        if j == p {
            continue;
        }
        let (ch_incl, ch_excl) = solve(j, i, g);
        assert_eq!(ch_incl.len(), ch_excl.len());
        let n = incl.len() + ch_incl.len();
        let mut new_incl = vec![0; n];
        let mut new_excl = vec![0; n];
        for c in 0..incl.len() {
            for cc in 0..ch_incl.len() {
                if c + cc >= 1 {
                    add!(new_incl[c + cc - 1], incl[c] * ch_incl[cc] % M);
                }
                add!(new_incl[c + cc], incl[c] * ch_excl[cc] % M);
                add!(new_excl[c + cc], excl[c] * (ch_incl[cc] + ch_excl[cc]) % M);
            }
        }
        incl = new_incl;
        excl = new_excl;
    }
    (incl, excl)
}

fn main() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1); n - 1],
    };

    let mut g = vec![vec![]; n];
    for (u, v) in edges {
        g[u].push(v);
        g[v].push(u);
    }

    let (incl, excl) = solve(0, std::usize::MAX, &g);
    for x in 1..=n {
        println!("{}", (incl[x] + excl[x]) % M);
    }
}
