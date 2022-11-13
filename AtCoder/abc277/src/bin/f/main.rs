use std::collections::HashMap;

use proconio::input;
use topological_sort::topological_sort;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    };

    let mut min_max = Vec::new();
    for i in 0..h {
        let values = a[i].iter().copied().filter(|&x| x >= 1).collect::<Vec<_>>();
        if values.is_empty() {
            continue;
        }
        let min = values.iter().min().copied().unwrap();
        let max = values.iter().max().copied().unwrap();
        min_max.push((min, max));
    }
    min_max.sort();
    for w in min_max.windows(2) {
        let (_, max1) = w[0];
        let (min2, _) = w[1];
        if max1 <= min2 {
            // ok
        } else {
            println!("No");
            return;
        }
    }

    let mut node = w;
    let mut edges = Vec::new();
    for i in 0..h {
        let mut values = Vec::new();
        let mut index = HashMap::<usize, Vec<usize>>::new();
        for j in 0..w {
            if a[i][j] >= 1 {
                values.push(a[i][j]);
                index.entry(a[i][j]).or_insert(Vec::new()).push(j);
            }
        }
        values.sort();
        values.dedup();
        for w in values.windows(2) {
            for &j in &index[&w[0]] {
                edges.push((j, node));
            }
            for &j in &index[&w[1]] {
                edges.push((node, j));
            }
            node += 1;
        }
    }

    if topological_sort(node, &edges).is_some() {
        println!("Yes");
    } else {
        println!("No");
    }
}
