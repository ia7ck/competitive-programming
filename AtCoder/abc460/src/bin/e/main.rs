use proconio::input;

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
        };

        solve(n, m);
    }
}

const P: usize = 998_244_353;

macro_rules! add {
    ($a: expr, $b: expr) => {
        $a = (($a) % P + ($b) % P) % P;
    };
}

fn solve(n: usize, m: usize) {
    let mut ans = 0;

    for k in 1..=n.to_string().len() as u32 {
        // k = 3
        // y = 100, 101, ..., 999
        let lb = 10_usize.pow(k - 1);
        let ub = (lb * 10 - 1).min(n);
        assert!(lb <= ub);
        let y = (ub - lb + 1) % P;

        let g = gcd(10_usize.pow(k) - 1, m);
        if g == 1 {
            let x = n / m % P;
            add!(ans, x * y);
        } else {
            let x = n / (m / g) % P;
            add!(ans, x * y);
        }
        // dbg!(n, m, k, g, lb, ub, x, y);
    }

    println!("{}", ans);
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}

#[allow(unused)]
fn mpow(a: usize, x: u32, m: usize) -> usize {
    match x {
        0 => 1 % m,
        1 => a % m,
        x if x % 2 == 0 => mpow(a * a % m, x / 2, m),
        _ => a * mpow(a, x - 1, m) % m,
    }
}
