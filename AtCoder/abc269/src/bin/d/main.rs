use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        xy: [(i32, i32); n],
    };

    let mut seen = vec![false; n];
    let mut ans = 0;
    for i in 0..n {
        if seen[i] {
            continue;
        }
        ans += 1;
        let mut que = VecDeque::new();
        seen[i] = true;
        que.push_back(i);
        while let Some(cur) = que.pop_front() {
            for next in 0..n {
                if seen[next] {
                    continue;
                }
                let (cx, cy) = xy[cur];
                let (nx, ny) = xy[next];
                let adj = [
                    (cx - 1, cy - 1),
                    (cx - 1, cy),
                    (cx, cy - 1),
                    (cx, cy + 1),
                    (cx + 1, cy),
                    (cx + 1, cy + 1),
                ];
                if adj.contains(&(nx, ny)) {
                    seen[next] = true;
                    que.push_back(next);
                }
            }
        }
    }

    println!("{}", ans);
}
