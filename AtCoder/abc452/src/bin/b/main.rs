use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    };

    let mut ans = vec![vec!['.'; w]; h];
    for i in [0, h - 1] {
        for j in 0..w {
            ans[i][j] = '#';
        }
    }
    for j in [0, w - 1] {
        for i in 0..h {
            ans[i][j] = '#';
        }
    }

    for row in ans {
        println!("{}", row.iter().collect::<String>());
    }
}
