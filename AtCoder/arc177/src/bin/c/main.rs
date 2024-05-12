use std::collections::VecDeque;

use grid_search::around;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        c: [Chars; n],
    };

    // 条件1のみ満たすような紫色マスの個数最小値c1、は求められる
    // 条件2のみ～c2、も同じ
    // この問題では同時に満たさないといけない
    //
    // (1, 1) -> (N, N), (1, N) -> (N, 1) の経路で
    // 交差するマスが1個以上ある
    // そこはかならず紫色マスにする
    // ……これ関係ある？
    //
    // c1 + c2 が答えの上界、実は達成できる？
    // (1, 1) -> (N, N) で赤→紫に変えたマスで
    // (1, N) -> (N, 1) が得することはない

    let red = bfs(n, &c, (0, 0), 'R', (n - 1, n - 1));
    let blue = bfs(n, &c, (0, n - 1), 'B', (n - 1, 0));
    dbg!(red);
    dbg!(blue);
    println!("{}", red + blue);
}

fn bfs(n: usize, c: &Vec<Vec<char>>, (si, sj): (usize, usize), start_color: char, (gi, gj): (usize, usize)) -> usize {
    const INF: usize = usize::MAX / 2;
    let mut dist = vec![vec![INF; n]; n];
    let mut que = VecDeque::new();
    dist[si][sj] = usize::from(c[si][sj] != start_color);
    que.push_back((si, sj));
    while let Some((i, j)) = que.pop_front() {
        for (ni, nj) in
            around(i, j)
                .y_range(0..n)
                .x_range(0..n)
                .directions(&[(-1, 0), (0, -1), (1, 0), (0, 1)])
        {
            if dist[ni][nj] == INF {
                let differ = c[ni][nj] != start_color;
                dist[ni][nj] = dist[i][j] + usize::from(differ);
                if differ {
                    que.push_back((ni, nj));
                } else {
                    que.push_front((ni, nj));
                }
            }
        }
    }
    dist[gi][gj]
}
