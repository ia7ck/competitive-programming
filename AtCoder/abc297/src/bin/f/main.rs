use factorials::Factorial;
use mod_int::ModInt998244353;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
    };

    type Mint = ModInt998244353;
    let f = Factorial::new(h * w + 1, 998244353);
    let g = |a: usize, b: usize| {
        if a * b < k {
            Mint::new(0)
        } else {
            Mint::from(f.binomial(a * b, k))
        }
    };
    let mut ans = Mint::new(0);
    for hh in 1..=h {
        for ww in 1..=w {
            let mut c = g(hh, ww);
            if hh >= 2 {
                c -= g(hh - 1, ww) * 2;
            }
            if ww >= 2 {
                c -= g(hh, ww - 1) * 2;
            }
            if hh >= 3 {
                c += g(hh - 2, ww);
            }
            if ww >= 3 {
                c += g(hh, ww - 2);
            }
            if hh >= 2 && ww >= 2 {
                c += g(hh - 1, ww - 1) * 4;
            }
            if hh >= 2 && ww >= 3 {
                c -= g(hh - 1, ww - 2) * 2;
            }
            if hh >= 3 && ww >= 2 {
                c -= g(hh - 2, ww - 1) * 2;
            }
            if hh >= 3 && ww >= 3 {
                c += g(hh - 2, ww - 2);
            }
            ans += c * (h - hh + 1) * (w - ww + 1) * hh * ww;
        }
    }

    ans /= f.binomial(h * w, k);
    println!("{}", ans.val());
}
