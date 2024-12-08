use mod_int::ModInt998244353;
use proconio::input;

type Mint = ModInt998244353;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    if a[0] == 0 || a[n - 1] == (n - 1) % 2 {
        println!("0");
        return;
    }

    for i in 1..(n - 1) {
        if a[i] == i % 2 && a[i + 1] == (i + 1) % 2 {
            println!("0");
            return;
        }
    }

    let mut l = 1;
    let mut ls = Vec::new();
    for i in 1..n {
        if a[i] == i % 2 {
            assert_ne!(a[i - 1], (i - 1) % 2);
            l += 1;
        } else if a[i - 1] == (i - 1) % 2 {
            assert_ne!(a[i], i % 2);
            l += 1;
        } else {
            if l >= 3 {
                ls.push(l);
            }
            l = 1;
        }
    }
    if l >= 3 {
        ls.push(l);
    }
    for i in 0..ls.len() {
        assert_eq!(ls[i] % 2, 1);
        ls[i] -= 2;
    }

    let mut f = vec![Mint::new(0); n + 1];
    let mut f2 = vec![Mint::new(0); n + 1];
    f[0] = 1.into();
    f2[0] = 1.into();
    for i in 1..=n {
        if i <= 2 {
            f[i] = Mint::from(i);
            f2[i] = Mint::from(i);
        } else {
            f[i] = f[i - 1] * Mint::from(i);
            f2[i] = f2[i - 2] * Mint::from(i);
        }
    }

    let s = ls.iter().fold(Mint::new(1), |acc, &l| acc * f2[l]);
    let p = f[ls.iter().map(|&l| (l + 1) / 2).sum::<usize>()];
    let q = ls.iter().fold(Mint::new(1), |acc, &l| acc * f[(l + 1) / 2]);
    let ans = s * p / q;
    println!("{}", ans.val());
}
