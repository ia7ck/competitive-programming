use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
    };

    let mut right_down = vec![0; n];
    let mut right_up = vec![0; n];
    for i in (0..n).rev() {
        right_down[i] = i;
        right_up[i] = i;
        if i + 1 < n {
            if p[i] > p[i + 1] {
                right_down[i] = right_down[i + 1];
            }
            if p[i] < p[i + 1] {
                right_up[i] = right_up[i + 1];
            }
        }
    }
    let mut left_down = vec![0; n];
    for i in 0..n {
        left_down[i] = i;
        if i > 0 {
            if p[i - 1] < p[i] {
                left_down[i] = left_down[i - 1];
            }
        }
    }

    
    let mut ans = 0_usize;
    for x2 in 0..n {
        let x1 = left_down[x2];
        let x3 = right_down[x2];
        let x4 = right_up[x3];
        if x1 < x2 && x2 < x3 && x3 < x4 {
            // p[x1] < p[x2] > p[x3] < p[x4]
            ans += (x2 - x1) * (x4 - x3);
        }
    }

    println!("{}", ans);
}
