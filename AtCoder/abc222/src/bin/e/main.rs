use input_i_scanner::{scan_with, InputIScanner};
use mod_int::ModInt998244353;
use std::collections::HashMap;

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let (n, m) = scan_with!(_i_i, (usize, usize));
    let k = scan_with!(_i_i, i32);
    let a = scan_with!(_i_i, usize; m);
    let edges = scan_with!(_i_i, (usize, usize); n - 1);

    let mut g = vec![vec![]; n];
    for (i, &(u, v)) in edges.iter().enumerate() {
        g[u - 1].push((i, v - 1));
        g[v - 1].push((i, u - 1));
    }
    let mut par = vec![(!0, !0); n];
    let mut dep = vec![0; n];
    dfs(0, !0, &g, &mut par, &mut dep);
    let mut c = vec![0; n - 1];
    for w in a.windows(2) {
        let u = w[0] - 1;
        let v = w[1] - 1;
        let (mut u, mut v) = if dep[u] >= dep[v] { (u, v) } else { (v, u) };
        while dep[u] != dep[v] {
            let (e_id, p) = par[u];
            c[e_id] += 1;
            u = p;
        }
        while u != v {
            let (u_e_id, u_p) = par[u];
            let (v_e_id, v_p) = par[v];
            c[u_e_id] += 1;
            c[v_e_id] += 1;
            u = u_p;
            v = v_p;
        }
    }
    // eprintln!("{:?}", c);
    type Mint = ModInt998244353;
    let mut dp: HashMap<i32, Mint> = HashMap::new();
    dp.insert(0, Mint::new(1));
    for c in c {
        let mut nxt = HashMap::new();
        for (key, val) in dp {
            nxt.entry(key + c)
                .and_modify(|e| {
                    *e += val;
                })
                .or_insert(val);
            nxt.entry(key - c)
                .and_modify(|e| {
                    *e += val;
                })
                .or_insert(val);
        }
        dp = nxt;
    }
    if let Some(ans) = dp.get(&k) {
        println!("{}", ans.val());
    } else {
        println!("0");
    }
}

fn dfs(
    u: usize,
    p: usize,
    g: &Vec<Vec<(usize, usize)>>,
    par: &mut Vec<(usize, usize)>,
    dep: &mut Vec<usize>,
) {
    for &(e_id, v) in &g[u] {
        if v != p {
            par[v] = (e_id, u);
            dep[v] = dep[u] + 1;
            dfs(v, u, g, par, dep);
        }
    }
}
