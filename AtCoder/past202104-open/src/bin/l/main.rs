use proconio::input;

fn visit(i: usize, g: &Vec<Vec<usize>>, done: &mut Vec<bool>) {
    if done[i] {
        return;
    }
    done[i] = true;
    for &j in &g[i] {
        visit(j, g, done);
    }
}

fn min_span_tree(n: usize, edges: &[(usize, usize, f64)]) -> f64 {
    let mut edges = edges.to_vec();
    edges.sort_by(|&(_, _, w1), &(_, _, w2)| w1.partial_cmp(&w2).unwrap());
    let mut sum = 0.0;
    let mut g = vec![vec![]; n];
    for (u, v, w) in edges {
        let mut visited = vec![false; n];
        visit(u, &g, &mut visited);
        if !visited[v] {
            sum += w;
            g[u].push(v);
            g[v].push(u);
        }
    }
    sum
}

fn main() {
    input! {
        n: usize,
        m: usize,
        p_xy: [(f64, f64); n],
        c_xyr: [(f64, f64, f64); m],
    };

    let mut ans = 1e18_f64;
    for bits in 0..(1 << m) {
        let mut xyr = Vec::new();
        for &(x, y) in &p_xy {
            xyr.push((x, y, 0.0));
        }
        for i in 0..m {
            if bits >> i & 1 == 1 {
                xyr.push(c_xyr[i]);
            }
        }
        let mut edges = Vec::new();
        for i in 0..xyr.len() {
            for j in (i + 1)..xyr.len() {
                let (x, y, r) = xyr[i];
                let (xx, yy, rr) = xyr[j];
                let d = (x - xx).hypot(y - yy);
                let weight = if d > r + rr {
                    // 離れ
                    d - (r + rr)
                } else if (r - rr).abs() > d {
                    // 包含
                    (r - rr).abs() - d
                } else {
                    // 交わる
                    0.0
                };
                edges.push((i, j, weight));
            }
        }
        ans = ans.min(min_span_tree(xyr.len(), &edges));
    }

    println!("{}", ans);
}
