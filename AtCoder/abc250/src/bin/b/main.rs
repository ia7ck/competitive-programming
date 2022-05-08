use join::Join;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    };

    let mut ans = vec![vec!['?'; b * n]; a * n];
    for i in 0..(a * n) {
        for j in 0..(b * n) {
            let u = i % (a * 2) < a;
            let v = j % (b * 2) < b;
            let c = match (u, v) {
                (true, true) | (false, false) => '.',
                (true, false) | (false, true) => '#',
            };
            ans[i][j] = c;
        }
    }

    for ans in ans {
        println!("{}", ans.iter().join(""));
    }
}
