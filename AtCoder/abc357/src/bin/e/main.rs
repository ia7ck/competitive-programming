use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    };

    let mut in_deg = vec![0; n];
    for &x in &a {
        in_deg[x] += 1;
    }

    let mut seen = vec![false; n];
    let mut reach = vec![0; n];
    let mut ans = 0;
    for s in 0..n {
        if in_deg[s] == 0 {
            let mut x = s;
            let mut hist = Vec::new();
            while seen[x] == false {
                seen[x] = true;
                hist.push(x);
                x = a[x];
            }
            match hist.iter().position(|&y| y == x) {
                Some(p) => {
                    for i in 0..p {
                        reach[hist[i]] = hist.len() - i;
                        ans += hist.len() - i;
                    }
                    let cycle = &hist[p..];
                    for &y in cycle {
                        reach[y] = cycle.len();
                        ans += cycle.len();
                    }
                }
                None => {
                    let len = reach[x];
                    for i in 0..hist.len() {
                        reach[hist[i]] = hist.len() - i + len;
                        ans += hist.len() - i + len;
                    }
                }
            }
        }
    }

    for s in 0..n {
        if seen[s] == false {
            let mut x = s;
            let mut cycle = Vec::new();
            while seen[x] == false {
                seen[x] = true;
                cycle.push(x);
                x = a[x];
            }
            let len = cycle.len();
            assert_ne!(len, 0);
            ans += len * len;
        }
    }

    println!("{}", ans);
}
