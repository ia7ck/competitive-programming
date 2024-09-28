use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1, i64); m],
    };

    let mut g = vec![vec![]; n];
    for &(u, v, w) in &edges {
        g[u].push((v, w));
        g[v].push((u, -w));
    }

    let mut x = vec![Option::<i64>::None; n];
    for i in 0..n {
        if x[i].is_none() {
            x[i] = Some(0);
            dfs(i, &g, &mut x);
        }
    }

    let x = x.into_iter().map(Option::unwrap).collect::<Vec<_>>();
    for &(u, v, w) in &edges {
        assert_eq!(x[v] - x[u], w);
    }

    println!(
        "{}",
        x.iter()
            .map(|&x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

fn dfs(i: usize, g: &Vec<Vec<(usize, i64)>>, x: &mut Vec<Option<i64>>) {
    let x_i = x[i].unwrap();
    for &(j, w) in &g[i] {
        // x[j] - x[i] = w
        match x[j] {
            Some(x_j) => {
                assert_eq!(x_j, x_i + w);
            }
            None => {
                x[j] = Some(x_i + w);
                dfs(j, g, x);
            }
        }
    }
}
