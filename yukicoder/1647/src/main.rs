use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let h: usize = rd.get();
    let w: usize = rd.get();
    let n: usize = rd.get();
    let mut edges = Vec::new();
    for _ in 0..n {
        let x: usize = rd.get();
        let y: usize = rd.get();
        edges.push((x - 1, h + (y - 1)));
    }
    if let Some(mut cycle) = detect_cycle_undirected(h + w, edges.iter().copied()) {
        assert!(cycle.len() >= 4);
        let (_, y0) = edges[cycle[0]];
        let (_, y1) = edges[cycle[1]];
        let (_, y2) = edges[cycle[2]];
        if y0 != y1 {
            assert_eq!(y1, y2);
            cycle.rotate_left(1);
        }
        println!("{}", cycle.len());
        for t in cycle {
            print!("{} ", t + 1);
        }
        return;
    }
    println!("-1");
}

fn detect_cycle_undirected(
    n: usize,
    edges: impl Iterator<Item = (usize, usize)>,
) -> Option<Vec<usize>> {
    fn dfs(
        curr: usize,
        prev: usize,
        g: &Vec<Vec<(usize, usize)>>,
        seen: &mut Vec<bool>,
        parent: &mut Vec<(usize, usize)>,
    ) -> Option<(usize, usize)> {
        seen[curr] = true;
        for &(nxt, idx) in &g[curr] {
            if nxt == prev {
                continue;
            }
            if seen[nxt] {
                return Some((nxt, curr));
            }
            parent[nxt] = (curr, idx);
            if let Some((start, end)) = dfs(nxt, curr, g, seen, parent) {
                return Some((start, end));
            }
        }
        None
    }

    let edges: Vec<(usize, usize)> = edges.collect();
    let mut g = vec![vec![]; n];
    for (idx, &(u, v)) in edges.iter().enumerate() {
        g[u].push((v, idx));
        g[v].push((u, idx));
    }
    let mut seen = vec![false; n];
    let mut parent = vec![(!0, !0); n];

    for v in 0..n {
        if seen[v] {
            continue;
        }
        if let Some((cycle_start, cycle_end)) = dfs(v, !0, &g, &mut seen, &mut parent) {
            let mut cycle = Vec::new();
            {
                let mut curr = cycle_end;
                while curr != cycle_start {
                    let (par, idx) = parent[curr];
                    cycle.push(idx);
                    curr = par;
                }
            }
            // cycle_end <- parent[cycle_end] <- parent[parent[cycle_end]] <- ... <- cycle_start
            for (idx, &(u, v)) in edges.iter().enumerate() {
                if (u, v) == (cycle_start, cycle_end) || (u, v) == (cycle_end, cycle_start) {
                    cycle.push(idx);
                    return Some(cycle);
                }
            }
            unreachable!();
        }
    }
    None
}
