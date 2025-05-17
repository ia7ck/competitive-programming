use proconio::{input, marker::Usize1};
use segment_tree::SegmentTree;

fn main() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1); n - 1],
        q: usize,
    };

    let mut g = vec![vec![]; n];
    for &(u, v) in &edges {
        g[u].push(v);
        g[v].push(u);
    }
    let root = 0;
    let mut tour = vec![];
    let mut parent = vec![usize::MAX; n];
    dfs(root, n, &g, &mut tour, &mut parent);
    let mut pos = vec![vec![]; n];
    for i in 0..tour.len() {
        pos[tour[i]].push(i);
    }
    for i in 0..n {
        assert_eq!(pos[i].len(), 2);
    }

    let mut seg = SegmentTree::new(n * 2, 0_u64, |a, b| a + b);
    for i in 0..n {
        seg.set(pos[i][0], 1);
    }

    for _ in 0..q {
        input! {
            op: u8,
        };
        if op == 1 {
            input! {
                x: Usize1,
                w: u64,
            };
            seg.update(pos[x][0], |w0| w0 + w);
        } else {
            input! {
                y: Usize1,
            };
            let (u, v) = edges[y];
            let c = if parent[u] == v { u } else { v };
            let sum_r = seg.fold(pos[root][0]..=pos[root][1]);
            let sum_c = seg.fold(pos[c][0]..=pos[c][1]);
            assert!(sum_r >= sum_c);
            let ans = (sum_r - sum_c).abs_diff(sum_c);
            println!("{}", ans);
        }
    }
}

fn dfs(i: usize, p: usize, g: &Vec<Vec<usize>>, tour: &mut Vec<usize>, parent: &mut Vec<usize>) {
    tour.push(i);
    for &j in &g[i] {
        if j != p {
            parent[j] = i;
            dfs(j, i, g, tour, parent);
        }
    }
    tour.push(i);
}
