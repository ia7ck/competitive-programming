use proconio::input;

fn main() {
    input! {
        t: usize,
    };
    for _ in 0..t {
        input! {
            n: u64,
        };
        solve(n);
    }
}

fn solve(n: u64) {
    let s: Vec<char> = n.to_string().chars().collect();

    let mut cand = Vec::new();
    cand.push("9".repeat(s.len() - 1).parse::<u64>().unwrap());

    for i in 0..s.len() {
        let pat = &s[..=i];
        let len = pat.len();
        if s.len() % len == 0 && s.len() / len >= 2 {
            let pat: Vec<u64> = pat.iter().map(|&ch| ch as u64 - '0' as u64).collect();
            let orig = pat[len - 1];
            let rng = if i == 0 {
                1..=orig
            } else {
                0..=9
            };
            for last in rng {
                let mut p = pat.clone();
                p[len - 1] = last;
                if orig < last {
                    assert!(i >= 1);
                    assert!(len >= 2);
                    p[len - 2] -= 1;
                }
                let mut x = 0;
                for _ in 0..(s.len() / len) {
                    for &d in &p {
                        x = x * 10 + d;
                    }
                }
                if x <= n {
                    cand.push(x);
                }
            }
        }
    }

    let ans = cand.iter().max().unwrap();
    println!("{}", ans);
}
