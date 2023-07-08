use proconio::{input, marker::Usize1};

fn solve(i: usize, g: &Vec<Vec<usize>>, y: &Vec<u64>, acc: u64, cover: &mut Vec<bool>) {
    // eprintln!("i = {}, y[i] = {}, acc = {}",i, y[i], acc);
    cover[i] = y[i].max(acc) >= 1;
    for &j in &g[i] {
        solve(
            j,
            g,
            y,
            y[i].max(acc.saturating_sub(1)),
            cover,
        );
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        p: [Usize1; n - 1],
        xy: [(Usize1, u64); m],
    };

    let mut y = vec![0; n];
    for (x, y_) in xy {
        y[x] = y[x].max(y_);
    }

    let mut g = vec![vec![]; n];
    for i in 1..n {
        g[p[i - 1]].push(i);
    }
    
    let mut cover = vec![false; n];
    solve(0, &g, &y, 0, &mut cover);
    let ans = cover.iter().filter(|&&f| f).count();
    println!("{}", ans);
}
