use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        edges: [(Usize1, Usize1, u64); m],
    };

    let mut queries = Vec::new();
    for _ in 0..q {
        input! {
            op: u8,
        };
        if op == 1 {
            input! {
                i: Usize1,
            };
            queries.push(Query::Stop(i));
        } else {
            input! {
                x: Usize1,
                y: Usize1,
            };
            queries.push(Query::Distance(x, y));
        }
    }

    const INF: u64 = u64::MAX / 3;
    let mut dist = vec![vec![INF; n]; n];

    for i in 0..n {
        dist[i][i] = 0;
    }

    let mut initial = vec![true; m];
    for query in &queries {
        if let Query::Stop(i) = query {
            initial[*i] = false;
        }
    }
    for i in 0..m {
        if initial[i] {
            let (u, v, cost) = edges[i];
            dist[u][v] = dist[u][v].min(cost);
            dist[v][u] = dist[v][u].min(cost);
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
            }
        }
    }

    let mut ans = Vec::new();
    for query in queries.into_iter().rev() {
        match query {
            Query::Stop(i) => {
                let (u, v, cost) = edges[i];
                for x in 0..n {
                    for y in 0..n {
                        dist[x][y] = dist[x][y].min(dist[x][u] + cost + dist[v][y]);
                        dist[x][y] = dist[x][y].min(dist[x][v] + cost + dist[u][y]);
                        dist[y][x] = dist[y][x].min(dist[y][u] + cost + dist[v][x]);
                        dist[y][x] = dist[y][x].min(dist[y][v] + cost + dist[u][x]);
                    }
                }
            }
            Query::Distance(x, y) => {
                if dist[x][y] == INF {
                    ans.push(-1);
                } else {
                    ans.push(dist[x][y] as i64);
                }
            }
        }
    }
    ans.reverse();

    for ans in ans {
        println!("{}", ans);
    }
}

enum Query {
    Stop(usize),
    Distance(usize, usize),
}
