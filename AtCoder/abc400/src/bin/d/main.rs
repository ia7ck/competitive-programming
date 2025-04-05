use proconio::{
    input,
    marker::{Chars, Usize1},
};

use dijkstra::{dijkstra, ConstEdge};
use grid_search::around;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
        (sy, sx, gy, gx): (Usize1, Usize1, Usize1, Usize1),
    };

    let node_id = |y: usize, x: usize| y * w + x;

    let mut edges = Vec::new();
    for y in 0..h {
        for x in 0..w {
            for (ny, nx) in around(y, x).y_range(0..h).x_range(0..w).directions(&[
                (-1, 0),
                (1, 0),
                (0, -1),
                (0, 1),
            ]) {
                if s[ny][nx] == '.' {
                    edges.push(ConstEdge::new(node_id(y, x), node_id(ny, nx), 0));
                } else {
                    // kick
                    edges.push(ConstEdge::new(node_id(y, x), node_id(ny, nx), 1));
                }
            }

            for (ny, nx) in around(y, x).y_range(0..h).x_range(0..w).directions(&[
                (-2, 0),
                (2, 0),
                (0, -2),
                (0, 2),
            ]) {
                // kick
                edges.push(ConstEdge::new(node_id(y, x), node_id(ny, nx), 1));
            }
        }
    }
    let (dist, _) = dijkstra(h * w, &edges, node_id(sy, sx));
    let ans = dist[node_id(gy, gx)].unwrap();
    println!("{}", ans);
}
