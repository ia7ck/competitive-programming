use proconio::{input, marker::Bytes};

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
        s: Bytes,
    };

    let mut pos_by_c = vec![vec![]; 26];
    for i in 0..n {
        pos_by_c[usize::from(s[i] - b'a')].push(i);
    }

    let mut ans = 0;
    for c in 0..26 {
        let sub_r = f(&pos_by_c[c], r);
        let sub_l = f(&pos_by_c[c], l - 1);
        ans += sub_r - sub_l;
    }

    println!("{}", ans);
}

// j - i <= w
fn f(pos: &Vec<usize>, w: usize) -> usize {
    let mut ans = 0;
    let mut l = 0;
    for r in 0..pos.len() {
        assert!(l <= r);
        while pos[r] - pos[l] > w {
            l += 1;
        }
        assert!(l <= r);

        ans += r - l + 1;
    }
    ans
}
