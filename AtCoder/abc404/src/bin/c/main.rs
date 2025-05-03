use proconio::{input, marker::Usize1};

use detect_cycle::detect_cycle_undirected;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    };

    if m != n {
        println!("No");
        return;
    }

    let ans = match detect_cycle_undirected(n, &edges) {
        Some(cycle) if cycle.len() == n => "Yes",
        _ => "No",
    };

    println!("{}", ans);
}
