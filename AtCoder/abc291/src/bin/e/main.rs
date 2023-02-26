use proconio::{input, marker::Usize1};
use topological_sort::topological_sort;

fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [(Usize1, Usize1); m],
    };

    let ord = topological_sort(n, &xy).unwrap();
    let mut in_deg = vec![0; n];
    for &(_, y) in &xy {
        in_deg[y] += 1;
    }
    if in_deg.iter().filter(|&&d| d == 0).count() >= 2 {
        println!("No");
        return;
    }
    let mut to = vec![vec![]; n];
    for &(x, y) in &xy {
        to[x].push(y);
    }
    for &u in &ord {
        let mut next = 0;
        for &v in &to[u] {
            assert!(in_deg[v] >= 1);
            in_deg[v] -= 1;
            if in_deg[v] == 0 {
                next += 1;
            }
        }
        if next >= 2 {
            println!("No");
            return;
        }
    }
    println!("Yes");
    let mut ans = vec![0; n];
    for i in 0..n {
        ans[ord[i]] = i;
    }
    for &(x, y) in &xy {
        assert!(ans[x] < ans[y]);
    }
    for i in 0..n {
        print!("{}", ans[i] + 1);
        if i + 1 < n {
            print!(" ");
        } else {
            println!();
        }
    }
}
