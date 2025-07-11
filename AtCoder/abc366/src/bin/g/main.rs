use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    };

    let mut a = vec![vec![0; n]; n];
    for (u, v) in edges {
        a[u][v] = 1;
        a[v][u] = 1;
    }

    // ax = 0, 0 < x[0] < 2^n, ..., 0 < x[n-1] < 2^n
    // ビットごとに見る
    // 解空間の基底を b[0], ..., b[k-1] とする, k <= n
    // x[0][0]   = b[0][0], ...,   x[0][k-1] = b[k-1][0],     x[0][k] = 0, ...
    // x[1][0]   = b[0][1], ...,   x[1][k-1] = b[k-1][1],     x[1][k] = 0, ...
    // ...
    // x[n-1][0] = b[0][n-1], ..., x[n-1][k-1] = b[k-1][n-1], x[n-1][k] = 0, ...

    let basis = {
        let mut a = a.clone();
        let mut i = 0;
        let mut pivot = vec![Option::<usize>::None; n];
        for j in 0..n {
            // 掃き出し法
            if a[i][j] == 0 {
                if let Some(p) = (i..n).find(|&p| a[p][j] == 1) {
                    a.swap(i, p);
                } else {
                    continue;
                }
            }
            assert_eq!(a[i][j], 1);
            for p in 0..n {
                if i != p && a[p][j] == 1 {
                    // p行目からi行目を引く
                    for q in 0..n {
                        a[p][q] ^= a[i][q];
                    }
                }
            }
            pivot[j] = Some(i);
            i += 1;
        }

        // ax = 0 の解空間の基底を求める
        let mut basis = Vec::new();
        for j in 0..n {
            if pivot[j].is_none() {
                let mut b = vec![0; n];
                b[j] = 1; // = -1 mod 2
                for k in 0..j {
                    if let Some(i) = pivot[k] {
                        b[k] = a[i][j];
                    }
                }
                basis.push(b);
            }
        }
        basis
    };

    let mut x = vec![0; n];
    for i in 0..n {
        for j in 0..basis.len() {
            if basis[j][i] == 1 {
                x[i] |= 1 << j;
            }
        }
    }

    if x.contains(&0) {
        println!("No");
    } else {
        println!("Yes");
        println!(
            "{}",
            x.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}
