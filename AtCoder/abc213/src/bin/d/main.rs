use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let mut g = vec![vec![]; n];
    for _ in 0..(n - 1) {
        let a: usize = rd.get();
        let b: usize = rd.get();
        g[a - 1].push(b - 1);
        g[b - 1].push(a - 1);
    }
    for v in 0..n {
        g[v].sort();
    }
    let mut ans = Vec::new();
    dfs(0, !0, &g, &mut ans);
    print!("{}", ans[0] + 1);
    for ans in &ans[1..] {
        print!(" {}", ans + 1);
    }
    println!();
}

fn dfs(i: usize, p: usize, g: &Vec<Vec<usize>>, ans: &mut Vec<usize>) {
    ans.push(i);
    for &j in &g[i] {
        if j != p {
            dfs(j, i, g, ans);
        }
    }
    if p != !0 {
        ans.push(p);
    }
}
