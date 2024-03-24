use proconio::{
    input,
    marker::{Chars, Usize1},
};

const M: u64 = 998244353;

fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
        s: Chars,
    };

    // left
    let mut ans_left = 1;
    let mut spoon = vec![true; n];
    for &p in &p {
        match (spoon[p], spoon[(p + 1) % n]) {
            (true, true) => {
                if s[p] == 'R' {
                    ans_left = 0;
                    break;
                }
            }
            (true, false) => {
                if s[p] == '?' {
                    ans_left *= 2;
                    ans_left %= M;
                }
            }
            (false, true) => {
                ans_left = 0;
                break;
            }
            _ => unreachable!(),
        }
        spoon[p] = false;
    }

    // right
    let mut ans_right = 1;
    let mut spoon = vec![true; n];
    for &p in &p {
        match (spoon[p], spoon[(p + 1) % n]) {
            (true, true) => {
                if s[p] == 'L' {
                    ans_right = 0;
                    break;
                }
            }
            (false, true) => {
                if s[p] == '?' {
                    ans_right *= 2;
                    ans_right %= M;
                }
            }
            (true, false) => {
                ans_right = 0;
                break;
            }
            _ => unreachable!(),
        }
        spoon[(p + 1) % n] = false;
    }

    dbg!(ans_left, ans_right);
    let ans = (ans_left + ans_right) % M;
    println!("{}", ans);
}
