use proconio::input;
use topological_sort::topological_sort;

fn main() {
    input! {
        n: usize,
        st: [(String, String); n],
    };

    let mut names = Vec::new();
    for (s, t) in &st {
        names.push(s);
        names.push(t);
    }
    names.sort();
    names.dedup();

    let n = names.len();
    let mut edges = Vec::new();
    for (s, t) in &st {
        let s = names.binary_search(&s).unwrap();
        let t = names.binary_search(&t).unwrap();
        edges.push((s, t));
    }

    if let Some(_) = topological_sort(n, &edges) {
        println!("Yes");
    } else {
        println!("No");
    }
}
