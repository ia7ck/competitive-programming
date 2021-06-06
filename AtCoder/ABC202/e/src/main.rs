use binary_search::BinarySearch;
use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let mut g = vec![vec![]; n];
    for i in 1..n {
        let p: usize = rd.get();
        g[p - 1].push(i);
    }

    let mut depth = vec![0; n];
    let mut in_time = vec![0; n];
    let mut out_time = vec![0; n];
    {
        let mut timer = 0;
        dfs(0, &g, &mut depth, &mut timer, &mut in_time, &mut out_time);
    }
    let mut a = vec![vec![]; n];
    for i in 0..n {
        a[depth[i]].push(in_time[i]);
    }
    for d in 0..n {
        a[d].sort();
    }
    let q: usize = rd.get();
    for _ in 0..q {
        let u: usize = rd.get();
        let d: usize = rd.get();
        let u = u - 1;
        let (in_t, out_t) = (in_time[u], out_time[u]);
        if a[d].is_empty() {
            println!("0");
            continue;
        }
        let ans = a[d].lower_bound(&out_t) - a[d].lower_bound(&in_t);
        println!("{}", ans);
    }
}

fn dfs(
    i: usize,
    g: &Vec<Vec<usize>>,
    depth: &mut Vec<usize>,
    timer: &mut usize,
    in_time: &mut Vec<usize>,
    out_time: &mut Vec<usize>,
) {
    in_time[i] = *timer;
    *timer += 1;
    for &j in &g[i] {
        depth[j] = depth[i] + 1;
        dfs(j, g, depth, timer, in_time, out_time);
    }
    out_time[i] = *timer;
    *timer += 1;
}
