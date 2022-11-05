use join::Join;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize); m],
    };

    let mut adj = vec![vec![]; n + 1];
    for (a, b) in edges {
        adj[a].push(b);
        adj[b].push(a);
    }

    for i in 1..=n {
        adj[i].sort();
        if adj[i].is_empty() {
            println!("0");
        } else {
            println!("{} {}", adj[i].len(), adj[i].iter().join(" "));
        }
    }
}
