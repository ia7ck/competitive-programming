use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n * 3],
    };

    let mut pos = vec![vec![]; n];
    for i in 0..(n * 3) {
        pos[a[i]].push(i);
    }
    let mut ans = (0..n).collect::<Vec<_>>();
    ans.sort_by_key(|&i| pos[i][1]);
    for i in 0..n {
        print!("{}", ans[i] + 1);
        if i + 1 < n {
            print!(" ");
        } else {
            print!("\n");
        }
    }
}
