use proconio::input;

fn main() {
    input! {
        n: usize,
        rects: [(usize, usize, usize, usize); n],
    };

    let mut sheet = vec![vec![0; 101]; 101];
    for (a, b, c, d) in rects {
        for y in c..d {
            for x in a..b {
                sheet[y][x] += 1;
            }
        }
    }
    let mut ans = 0;
    for y in 0..=100 {
        for x in 0..=100 {
            if sheet[y][x] >= 1 {
                ans += 1;
            };
        }
    }
    println!("{}", ans);
}
