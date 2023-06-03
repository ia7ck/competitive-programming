use mod_int::ModInt998244353;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    type Mint = ModInt998244353;
    let mut h = Vec::<(usize, Mint)>::new();
    for m in 1..n {
        if n % m != 0 {
            continue;
        }
        let mut fix = vec![false; m];
        for i in 0..n {
            fix[i % m] |= s[i] == '.';
        }
        let mut total = Mint::new(1);
        for i in 0..m {
            if fix[i] == false {
                total *= 2;
            }
        }
        let mut dup = Mint::new(0);
        for (k, c) in &h {
            if m % k != 0 {
                continue;
            }
            dup += *c;
        }
        h.push((m, total - dup));
    }

    let mut ans = Mint::new(0);
    for (p, c) in h {
        eprintln!("p = {}, c = {}", p, c.val());
        ans += c;
    }
    println!("{}", ans.val());
}

// 12
// ####.####.##

// 1
// ############

// 2
// ############

// 3
// ##?##?##?##?

// 4
// ##??##??##??

// 6
// ???##????##?
