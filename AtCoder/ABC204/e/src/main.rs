use floor_sqrt::floor_sqrt;
use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let m: usize = rd.get();
    let mut g = vec![vec![]; n];
    for _ in 0..m {
        let a: usize = rd.get();
        let b: usize = rd.get();
        let (a, b) = (a - 1, b - 1);
        let c: u64 = rd.get();
        let d: u64 = rd.get();
        g[a].push(Edge { to: b, c, d });
        g[b].push(Edge { to: a, c, d });
    }

    let d = dijkstra(&g, 0);
    if let Some(ans) = d[n - 1] {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Edge {
    to: usize,
    c: u64,
    d: u64,
}

#[allow(clippy::many_single_char_names)]
pub fn dijkstra(g: &[Vec<Edge>], s: usize) -> Vec<Option<u64>> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let n = g.len();
    let mut dist = vec![None; n];
    let mut q = BinaryHeap::new();
    dist[s] = Some(0);
    q.push((Reverse(0), s));
    while let Some((Reverse(d), v)) = q.pop() {
        match dist[v] {
            Some(dv) => {
                if dv < d {
                    continue;
                }
            }
            None => unreachable!(),
        }
        for e in &g[v] {
            let mut nexts = vec![d + e.c + e.d / (d + 1)];
            let sqrt_d = floor_sqrt(e.d);
            for dd in sqrt_d.saturating_sub(5)..=(sqrt_d + 5) {
                if dd >= d {
                    nexts.push(dd + e.c + e.d / (dd + 1));
                }
            }
            for next_d in nexts {
                match dist[e.to] {
                    Some(dt) if dt <= next_d => {
                        continue;
                    }
                    _ => {
                        dist[e.to] = Some(next_d);
                        q.push((Reverse(next_d), e.to));
                    }
                }
            }
        }
    }
    dist
}
