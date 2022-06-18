use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [usize; n],
        c: [u64; n],
    };

    let mut seen = vec![false; n];
    let mut ans = 0;
    for v in 0..n {
        if seen[v] {
            continue;
        }
        let mut route = Vec::new();
        let mut v = v;
        while !seen[v] {
            seen[v] = true;
            route.push(v);
            v = x[v] - 1;
        }
        if let Some(p) = route.iter().position(|&u| u == v) {
            let min = &route[p..].iter().map(|&u| c[u]).min().unwrap();
            ans += min;
        }
    }
    println!("{}", ans);
}
