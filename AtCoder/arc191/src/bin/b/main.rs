use proconio::input;

fn main() {
    if cfg!(debug_assertions) {
        for n in 1..20 {
            eprintln!("n = {n} = 0b{n:b}");
            let mut xs = Vec::new();
            for x in 1..1000 {
                if x ^ n == x % n {
                    xs.push(x);
                }
            }
            eprintln!(
                "x = [{}]",
                xs.iter()
                    .map(|x| format!("{x:b}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
    }

    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: u64,
            k: u64,
        };
        solve(n, k);
    }
}

fn solve(n: u64, k: u64) {
    let msb = (0..32).rfind(|&i| n >> i & 1 == 1).unwrap();
    let mut z = Vec::new();
    for i in 0..msb {
        if n >> i & 1 == 0 {
            z.push(i);
        }
    }
    if 1 << z.len() < k {
        println!("-1");
        return;
    }
    let mut ans = n;
    if let Some(msb) = (0..32).rfind(|&i| (k - 1) >> i & 1 == 1) {
        let k = (0..=msb).map(|i| (k - 1) >> i & 1).collect::<Vec<_>>();
        assert!(k.len() <= z.len());
        for (&z, &k) in z.iter().zip(&k) {
            assert!(ans >> z & 1 == 0);
            if k == 1 {
                ans ^= 1 << z;
            }
        }
    }
    println!("{}", ans);
}
