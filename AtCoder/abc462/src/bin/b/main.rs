use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [[Usize1]; n],
    };

    let mut ans = vec![vec![]; n];
    for i in 0..n {
        for &j in &a[i] {
            ans[j].push(i);
        }
    }
    for i in 0..n {
        ans[i].sort_unstable();
    }

    for ans in ans {
        print!("{}", ans.len());
        for ans in ans {
            print!(" {}", ans + 1);
        }
        println!();
    }
}
