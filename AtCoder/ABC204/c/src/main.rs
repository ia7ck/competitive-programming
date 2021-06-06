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
        g[a].push(b);
    }
    let solve = |start: usize| -> usize {
        let mut seen = vec![false; n];
        use std::collections::VecDeque;
        let mut q = VecDeque::new();
        seen[start] = true;
        q.push_back(start);
        while let Some(v) = q.pop_front() {
            for &u in &g[v] {
                if !seen[u] {
                    seen[u] = true;
                    q.push_back(u);
                }
            }
        }
        seen.iter().filter(|f| **f).count()
    };
    let mut ans = 0;
    for s in 0..n {
        ans += solve(s);
    }
    println!("{}", ans);
}
