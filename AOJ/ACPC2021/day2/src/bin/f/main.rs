use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let m: usize = rd.get();
    let t: u32 = rd.get();
    let p1: usize = rd.get();
    let p2: usize = rd.get();
    let p3: usize = rd.get();
    let mut g = vec![vec![]; n];
    for _ in 0..m {
        let a: usize = rd.get();
        let b: usize = rd.get();
        g[a - 1].push(b - 1);
        g[b - 1].push(a - 1);
    }

    let inf = std::u32::MAX / 2;
    let d1 = bfs(n, p1 - 1, &g, inf);
    let d2 = bfs(n, p2 - 1, &g, inf);
    let d3 = bfs(n, p3 - 1, &g, inf);

    let le_t = |lb: u32| -> bool {
        use std::collections::VecDeque;

        if d1[0].min(d2[0]).min(d3[0]) < lb {
            return false;
        }
        let mut d = vec![inf; n];
        let mut que = VecDeque::new();
        d[0] = 0;
        que.push_back(0);
        while let Some(u) = que.pop_front() {
            for &v in &g[u] {
                if d1[v].min(d2[v]).min(d3[v]) >= lb && d[v] == inf {
                    d[v] = d[u] + 1;
                    que.push_back(v);
                }
            }
        }
        d[n - 1] <= t
    };

    let mut ng = n as u32;
    let mut ok = 0;
    while ng - ok > 1 {
        let mid = (ng + ok) / 2;
        if le_t(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}

fn bfs(n: usize, s: usize, g: &Vec<Vec<usize>>, inf: u32) -> Vec<u32> {
    use std::collections::VecDeque;

    let mut d = vec![inf; n];
    let mut que = VecDeque::new();
    d[s] = 0;
    que.push_back(s);
    while let Some(u) = que.pop_front() {
        for &v in &g[u] {
            if d[v] == inf {
                d[v] = d[u] + 1;
                que.push_back(v);
            }
        }
    }
    d
}
