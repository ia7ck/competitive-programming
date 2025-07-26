use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize, u64); m],
        k: usize,
        t0: u64,
        d: [usize; k],
        q: usize,
    };

    const INF: u64 = u64::MAX / 3;
    let mut dist = vec![vec![INF; 1 + n]; 1 + n];
    for i in 0..(1 + n) {
        dist[i][i] = 0;
    }
    for (a, b, c) in edges {
        let c = c * 2;
        dist[a][b] = dist[a][b].min(c);
        dist[b][a] = dist[b][a].min(c);
    }
    for i in d {
        dist[0][i] = t0;
        dist[i][0] = t0;
    }
    for k in 0..(1 + n) {
        for i in 0..(1 + n) {
            for j in 0..(1 + n) {
                dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
            }
        }
    }

    for _ in 0..q {
        input! {
            op: u8,
        };
        if op == 1 {
            input! {
                x: usize,
                y: usize,
                c: u64,
            };
            let c = c * 2;
            dist[x][y] = dist[x][y].min(c);
            dist[y][x] = dist[y][x].min(c);
            for s in 0..(1 + n) {
                for t in 0..(1 + n) {
                    // s -> x -(c)-> y -> t
                    dist[s][t] = dist[s][t].min(dist[s][x] + c + dist[y][t]);

                    // s -> y -(c)-> x -> t
                    dist[s][t] = dist[s][t].min(dist[s][y] + c + dist[x][t]);
                }
            }
        } else if op == 2 {
            input! {
                x: usize,
            };
            dist[0][x] = t0;
            dist[x][0] = t0;
            for s in 0..(1 + n) {
                for t in 0..(1 + n) {
                    // s -> x -(t)-> 0 -> t
                    dist[s][t] = dist[s][t].min(dist[s][x] + t0 + dist[0][t]);

                    // s -> 0 -(t)-> x -> t
                    dist[s][t] = dist[s][t].min(dist[s][0] + t0 + dist[x][t]);
                }
            }
        } else {
            let mut ans = 0;
            for x in 1..(1 + n) {
                for y in 1..(1 + n) {
                    if dist[x][y] < INF {
                        ans += dist[x][y];
                    }
                }
            }

            println!("{}", ans / 2);
        }
    }
}
