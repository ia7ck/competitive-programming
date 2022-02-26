use binary_search::BinarySearch;
use coordinate_compression::CoordinateCompression;
use dijkstra::{dijkstra, ConstEdge};
use scanner_proc_macro::insert_scanner;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

#[insert_scanner]
fn main() {
    let (h, w, n) = scan!((usize, usize, usize));
    let (sy, sx) = scan!((usize, usize));
    let (gy, gx) = scan!((usize, usize));
    let walls = scan!((usize, usize); n);

    let wall_set: HashSet<(usize, usize)> = walls.iter().copied().collect();

    let mut yx = Vec::new();
    yx.push((sy, sx));
    let mut ys_by_x = HashMap::<usize, Vec<usize>>::new();
    let mut xs_by_y = HashMap::<usize, Vec<usize>>::new();
    for &(y, x) in &walls {
        if y - 1 >= 1 {
            if !wall_set.contains(&(y - 1, x)) {
                yx.push((y - 1, x));
            }
        }
        if y + 1 <= h {
            if !wall_set.contains(&(y + 1, x)) {
                yx.push((y + 1, x));
            }
        }
        if x - 1 >= 1 {
            if !wall_set.contains(&(y, x - 1)) {
                yx.push((y, x - 1));
            }
        }
        if x + 1 <= w {
            if !wall_set.contains(&(y, x + 1)) {
                yx.push((y, x + 1));
            }
        }
        ys_by_x.entry(x).or_insert(Vec::new()).push(y);
        xs_by_y.entry(y).or_insert(Vec::new()).push(x);
    }

    if !yx.contains(&(gy, gx)) {
        println!("-1");
        return;
    }

    for ys in ys_by_x.values_mut() {
        ys.sort();
        ys.dedup();
    }
    for xs in xs_by_y.values_mut() {
        xs.sort();
        xs.dedup();
    }

    let yx_comp = CoordinateCompression::from_iter(yx.iter().copied());
    let node_index = |y: usize, x: usize| -> usize { yx_comp.find_index(&(y, x)) };

    let mut edges = Vec::new();
    for &(y, x) in &yx {
        let v = node_index(y, x);
        if let Some(ys) = ys_by_x.get(&x) {
            let i = ys.lower_bound(&y);
            if i >= 1 {
                let ty = ys[i - 1] + 1;
                edges.push(ConstEdge::new(v, node_index(ty, x), 1_u64));
            }
            if i < ys.len() {
                let dy = ys[i] - 1;
                edges.push(ConstEdge::new(v, node_index(dy, x), 1));
            }
        }
        if let Some(xs) = xs_by_y.get(&y) {
            let i = xs.lower_bound(&x);
            if i >= 1 {
                let lx = xs[i - 1] + 1;
                edges.push(ConstEdge::new(v, node_index(y, lx), 1_u64));
            }
            if i < xs.len() {
                let rx = xs[i] - 1;
                edges.push(ConstEdge::new(v, node_index(y, rx), 1));
            }
        }
    }

    let (d, _) = dijkstra(yx_comp.size(), edges.iter().copied(), node_index(sy, sx));
    if let Some(ans) = d[node_index(gy, gx)] {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
