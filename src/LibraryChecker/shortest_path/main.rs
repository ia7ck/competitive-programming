use std::io::Read;

fn read<T: std::str::FromStr>() -> T {
    let token: String = std::io::stdin()
        .bytes()
        .map(|c| c.ok().unwrap() as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().unwrap()
}

#[derive(Debug, Clone)]
struct Edge {
    to: usize,
    cost: i64,
}

fn dijkstra(g: &Vec<Vec<Edge>>, s: usize, inf: i64) -> (Vec<i64>, Vec<Option<usize>>) {
    use std::cmp::Reverse;
    let n = g.len();
    let mut d = vec![inf; n];
    let mut prev = vec![None; n];
    let mut q = std::collections::BinaryHeap::new();
    d[s] = 0;
    q.push((Reverse(d[s]), s));
    while let Some((Reverse(c), v)) = q.pop() {
        if c > d[v] {
            continue;
        }
        for e in &g[v] {
            if c + e.cost < d[e.to] {
                d[e.to] = c + e.cost;
                prev[e.to] = Some(v);
                q.push((Reverse(d[e.to]), e.to));
            }
        }
    }
    (d, prev)
}

fn main() {
    let n: usize = read();
    let m: usize = read();
    let s: usize = read();
    let t: usize = read();
    let mut g = vec![vec![]; n];
    for _ in 0..m {
        let a: usize = read();
        let b: usize = read();
        let c: i64 = read();
        g[a].push(Edge { to: b, cost: c });
    }
    let inf = std::i64::MAX / 3;
    let (d, prev) = dijkstra(&g, s, inf);
    if d[t] == inf {
        println!("-1");
        return;
    }
    let route = {
        let mut route = vec![];
        let mut v = t;
        while v != s {
            let p = prev[v].unwrap();
            route.push((p, v));
            v = p;
        }
        route
    };

    println!("{} {}", d[t], route.len());
    println!(
        "{}",
        route
            .iter()
            .rev()
            .map(|(a, b)| format!("{} {}", a, b))
            .collect::<Vec<_>>()
            .join("\n")
    );
}
