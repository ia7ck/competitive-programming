use proconio::{input, marker::Usize1};

fn dfs(
    i: usize,
    g: &Vec<Vec<(usize, i64)>>,
    depth: &mut Vec<usize>,
    val: &mut Vec<i64>,
    xs: &mut Vec<i64>,
) -> bool {
    let mut result = true;
    for &(j, c) in &g[i] {
        if depth[j] == std::usize::MAX {
            depth[j] = depth[i] + 1;
            val[j] = c - val[i];
            dfs(j, g, depth, val, xs);
        } else {
            if depth[i] % 2 != depth[j] % 2 {
                // val[i] + val[j] = (S - x) + (T + x) = c
                //   or
                // val[i] + val[j] = (S + x) + (T - x) = c
                if val[i] + val[j] != c {
                    result = false;
                }
            } else {
                // val[i] + val[j] = (S - x) + (T - x) = c - x * 2
                //   or
                // val[i] + val[j] = (S + x) + (T + x) = c + x * 2
                let y = (val[i] + val[j]) - c;
                if y % 2 != 0 {
                    result = false;
                    continue;
                }
                xs.push((y / 2).abs());
            }
        }
    }
    result
}

fn main() {
    macro_rules! impossible {
        () => {
            println!("-1");
            return;
        };
    }
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, i64); m],
    };

    let mut g = vec![vec![]; n];
    for &(a, b, c) in &abc {
        g[a].push((b, c));
        g[b].push((a, c));
    }

    let mut depth = vec![std::usize::MAX; n];
    depth[0] = 0;
    let mut val = vec![0; n];
    let mut xs = Vec::new();
    let ok = dfs(0, &g, &mut depth, &mut val, &mut xs);
    if !ok {
        impossible!();
    }

    // eprintln!("depth = {:?}", depth);
    // eprintln!("val = {:?}", val);
    // eprintln!("xs = {:?}", xs);

    xs.sort();
    xs.dedup();
    if xs.len() >= 2 {
        impossible!();
    }

    let mut low = 0;
    let mut high = std::i64::MAX / 2;
    for i in 0..n {
        if depth[i] % 2 == 0 {
            // val[i] + x >= 0
            low = low.max(-val[i]);
        } else {
            // val[i] - x >= 0
            high = high.min(val[i]);
        }
    }

    eprintln!("low = {}, high = {}", low, high);

    let x = if xs.len() == 1 { xs[0] } else { low };
    if x < low || high < x {
        impossible!();
    }

    for i in 0..n {
        if depth[i] % 2 == 0 {
            println!("{}", val[i] + x);
        } else {
            println!("{}", val[i] - x);
        }
    }
}
