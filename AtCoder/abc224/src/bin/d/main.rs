use input_i_scanner::{scan_with, InputIScanner};
use std::collections::{HashMap, VecDeque};

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    const N: usize = 9;
    let m = scan_with!(_i_i, usize);
    // 9! = 362880
    let mut g = vec![vec![false; N]; N];
    for _ in 0..m {
        let (u, v) = scan_with!(_i_i, (usize, usize));
        g[u - 1][v - 1] = true;
        g[v - 1][u - 1] = true;
    }
    let p = scan_with!(_i_i, usize; N - 1);

    let mut a = [N - 1; N];
    for i in 0..(N - 1) {
        a[p[i] - 1] = i;
    }
    let mut d = HashMap::new();
    let mut que = VecDeque::new();
    d.insert(a, 0);
    que.push_back((a, 0_u32));
    while let Some((a, d_a)) = que.pop_front() {
        let mut t = N;
        for i in 0..N {
            if a[i] == N - 1 {
                t = i;
                break;
            }
        }
        assert_ne!(t, N);
        for u in 0..N {
            if a[u] != N - 1 && g[u][t] {
                // u -> t
                let mut b = a;
                b.swap(u, t);
                if d.contains_key(&b) {
                    continue;
                }
                d.insert(b, d_a + 1);
                que.push_back((b, d_a + 1));
            }
        }
    }
    if let Some(ans) = d.get(&[0, 1, 2, 3, 4, 5, 6, 7, 8]) {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
