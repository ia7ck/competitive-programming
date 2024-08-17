use least_prime_factors::least_prime_factors;
use proconio::{input, marker::Usize1};
use rand::{rngs::SmallRng, Rng, SeedableRng};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        queries: [(Usize1, Usize1); q],
    };

    let lpf = least_prime_factors(1_000_000 + 1);
    let mut f = Vec::new();
    for &x in &a {
        let mut g = Vec::new();
        let mut x = x;
        while x > 1 {
            let p = lpf[x];
            let mut c = 0;
            while x % p == 0 {
                x /= p;
                c += 1;
            }
            g.push((p, c));
        }
        f.push(g);
    }
    let mut rng = SmallRng::seed_from_u64(0);
    let mut h = vec![0; 1_000_000 + 1];
    let mut cumsum = vec![vec![0; n + 1]; 64];
    for i in 0..64 {
        for i in 0..h.len() {
            h[i] = rng.gen_range(0..3);
        }
        for j in 0..n {
            cumsum[i][j + 1] = cumsum[i][j];
            for &(p, c) in &f[j] {
                cumsum[i][j + 1] += h[p] * c;
                cumsum[i][j + 1] %= 3;
            }
        }
    }
    for (l, r) in queries {
        let cubic = (0..64).all(|i| (3 + cumsum[i][r + 1] - cumsum[i][l]) % 3 == 0);
        if cubic {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
