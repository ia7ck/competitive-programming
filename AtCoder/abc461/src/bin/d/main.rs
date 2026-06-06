use proconio::{input, marker::Bytes};

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        s: [Bytes; h],
    };

    let mut ans = solve(h, w, k, &s);
    if k >= 1 {
        let sub = solve(h, w, k - 1, &s);
        ans -= sub;
    }

    println!("{}", ans);
}

fn solve(h: usize, w: usize, k: usize, s: &Vec<Vec<u8>>) -> usize {
    let mut psum = vec![vec![0; w + 1]; h];
    for i in 0..h {
        for j in 0..w {
            let x = usize::from(s[i][j] - b'0');
            psum[i][j + 1] += psum[i][j] + x;
        }
    }

    let get = |r: usize, c1: usize, c2: usize| -> usize {
        assert!(r < h);
        assert!(c1 < w);
        assert!(c2 < w);
        assert!(c1 <= c2);

        psum[r][c2 + 1] - psum[r][c1]
    };

    let mut ans = 0;
    for c1 in 0..w {
        for c2 in c1..w {
            let mut sum = 0;
            let mut y = 0;
            for r2 in 0..h {
                sum += get(r2, c1, c2);
                while sum > k {
                    sum -= get(y, c1, c2);
                    y += 1;
                }
                if r2 >= y {
                    ans += r2 - y + 1;
                }
            }
        }
    }

    ans
}
