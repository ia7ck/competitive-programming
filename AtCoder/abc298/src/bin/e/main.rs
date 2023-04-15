use mod_int::ModInt998244353;
use proconio::input;

macro_rules! add {
    ($a: expr, $b: expr) => {
        $a = ($a + $b) % 998244353;
    };
}

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        p: usize,
        q: usize,
    };

    let mut dp_p = vec![vec![0; (p + 1) * (n + 1) + 1]; n + 1];
    let mut dp_q = vec![vec![0; (q + 1) * (n + 1) + 1]; n + 1];
    dp_p[0][0] = 1;
    dp_q[0][0] = 1;
    for j in 1..=p {
        dp_p[1][j] = 1;
    }
    for j in 1..=q {
        dp_q[1][j] = 1;
    }
    for i in 1..n {
        for j in 1..=(p + 1) * (n + 1) {
            for d in 1..=p {
                if j + d <= (p + 1) * (n + 1) {
                    add!(dp_p[i + 1][j + d], dp_p[i][j]);
                }
            }
        }
    }
    for i in 1..n {
        for j in 1..=(q + 1) * (n + 1) {
            for d in 1..=q {
                if j + d <= (q + 1) * (n + 1) {
                    add!(dp_q[i + 1][j + d], dp_q[i][j]);
                }
            }
        }
    }

    type Mint = ModInt998244353;
    let mut ans = Mint::new(0);

    for x in 1..=(n - a) {
        let mut s = 0;
        for j in 0..(n - a) {
            add!(s, dp_p[x - 1][j]);
        }
        let mut t = 0;
        for j in (n - a)..=(p * x) {
            add!(t, dp_p[x][j]);
        }
        let mut u = 0;
        for j in 0..(n - b) {
            add!(u, dp_q[x - 1][j]);
        }
        let pp = Mint::from(s) / Mint::from(p).pow((x - 1) as u32);
        let qq = Mint::from(t) / Mint::from(p).pow(x as u32);
        let rr = Mint::from(u) / Mint::from(q).pow((x - 1) as u32);
        ans += (qq - (Mint::new(1) - pp)) * rr;
    }

    println!("{}", ans.val());
}
