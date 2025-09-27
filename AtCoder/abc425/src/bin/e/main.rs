use proconio::input;

fn main() {
    input! {
        t: usize,
        m: u64,
    };

    let mut binom = vec![vec![0; 5001]; 5001];
    for i in 0..=5000 {
        binom[i][0] = 1;
        binom[i][i] = 1;
    }
    for i in 2..=5000 {
        for j in 1..=i {
            binom[i][j] = (binom[i - 1][j - 1] + binom[i - 1][j]) % m;
        }
    }

    for _ in 0..t {
        input! {
            n: usize,
            c: [usize; n],
        };

        solve(n, c, m, &binom);
    }
}

fn solve(_n: usize, c: Vec<usize>, m: u64, binom: &[Vec<u64>]) {
    let mut ans = 1;
    let mut a = c.iter().sum::<usize>();
    for c in c {
        assert!(a >= c);
        ans = ans * binom[a][c] % m;
        a -= c;
    }
    println!("{}", ans);
}
