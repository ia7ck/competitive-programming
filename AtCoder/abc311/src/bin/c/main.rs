use proconio::{input, marker::Usize1};
use detect_cycle::detect_cycle_directed;

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    };

    let mut edges = Vec::new();
    for i in 0..n {
        edges.push((i, a[i]));
    }
    let cycle = detect_cycle_directed(n, &edges).unwrap();
    let mut ans = Vec::new();
    for i in cycle {
        let (s, _) = edges[i];
        ans.push(s);
    }
    println!("{}", ans.len());
    for i in 0..ans.len() {
        print!("{}", ans[i] + 1);
        if i + 1 < ans.len() {
            print!(" ");
        } else {
            print!("\n");
        }
    }
}
