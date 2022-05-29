use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };

    let mut si = 0;
    let mut sj = 0;
    for i in 0..h {
        for j in 0.. w {
            if s[i][j] == 'o' {
                si = i;
                sj = j;
                break;
            }
        }
    }
    let mut gi = 0;
    let mut gj = 0;
    for i in (0..h).rev() {
        for j in (0.. w).rev() {
            if s[i][j] == 'o' {
                gi = i;
                gj = j;
                break;
            }
        }
    }

    let ans = (si.max(gi) - si.min(gi)) + (sj.max(gj) - sj.min(gj));
    println!("{}", ans);
}
