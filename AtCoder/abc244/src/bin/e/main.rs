use scanner_proc_macro::insert_scanner;

#[insert_scanner]
fn main() {
    let (n, m, k, s, t, x) = scan!((usize, usize, usize, usize, usize, usize));
    let (s, t, x) = (s - 1, t - 1, x - 1);
    let mut g = vec![vec![]; n];
    for _ in 0..m {
        let (u, v) = scan!((usize, usize));
        g[u - 1].push(v - 1);
        g[v - 1].push(u - 1);
    }

    let mo = 998244353_u64;

    let mut dp_even = vec![0; n];
    let mut dp_odd = vec![0; n];
    if s == x {
        dp_odd[s] = 1;
    } else {
        dp_even[s] = 1;
    }
    for _ in 0..k {
        let mut nxt_even = vec![0; n];
        let mut nxt_odd = vec![0; n];
        for u in 0..n {
            for &v in &g[u] {
                if v == x {
                    nxt_even[v] += dp_odd[u];
                    nxt_even[v] %= mo;
                    nxt_odd[v] += dp_even[u];
                    nxt_odd[v] %= mo;
                } else {
                    nxt_even[v] += dp_even[u];
                    nxt_even[v] %= mo;
                    nxt_odd[v] += dp_odd[u];
                    nxt_odd[v] %= mo;
                }
            }
        }
        dp_even = nxt_even;
        dp_odd = nxt_odd;
    }
    let ans = dp_even[t];
    println!("{}", ans);
}
