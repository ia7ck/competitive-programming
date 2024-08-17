use least_prime_factors::least_prime_factors;
use proconio::{input, marker::Usize1};

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

    let q_sqrt = f64::sqrt(q as f64) as usize;
    let mut ord = (0..q).collect::<Vec<_>>();
    ord.sort_by_key(|&i| {
        let (l, r) = queries[i];
        let j = l * q_sqrt / n;
        (j, if j % 2 == 0 { r } else { n - r })
    });

    struct S {
        // counter[p] := p の個数
        counter: Vec<usize>,
        // mod3[x] := counter[*] % 3 == x の個数
        mod3: [usize; 3],
    }

    let add = |state: &mut S, i: usize| {
        for &(p, c) in &f[i] {
            state.mod3[state.counter[p] % 3] -= 1;
            state.counter[p] += c;
            state.mod3[state.counter[p] % 3] += 1;
        }
    };

    let remove = |state: &mut S, i: usize| {
        for &(p, c) in &f[i] {
            state.mod3[state.counter[p] % 3] -= 1;
            state.counter[p] -= c;
            state.mod3[state.counter[p] % 3] += 1;
        }
    };

    let mut state = {
        let mut counter = vec![0; 1_000_000 + 1];
        let mut mod3 = [0; 3];
        for i in 0..n {
            for &(p, c) in &f[i] {
                counter[p] += c;
            }
        }
        for &c in &counter {
            mod3[c % 3] += 1;
        }
        S { counter, mod3 }
    };
    let (mut left, mut right) = (0, n - 1);
    let mut ans = vec![false; q];
    for i in ord {
        let (l, r) = queries[i];
        while l < left {
            left -= 1;
            add(&mut state, left);
        }
        while right < r {
            right += 1;
            add(&mut state, right);
        }
        while left < l {
            remove(&mut state, left);
            left += 1;
        }
        while r < right {
            remove(&mut state, right);
            right -= 1;
        }
        ans[i] = state.mod3[1] == 0 && state.mod3[2] == 0;
    }

    for ans in ans {
        if ans {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
