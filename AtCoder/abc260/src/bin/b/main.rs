use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        z: usize,
        a: [u8; n],
        b: [u8; n],
    };

    let mut math: Vec<usize> = (0..n).collect();
    // stable
    math.sort_by_key(|&i| Reverse(a[i]));
    let mut eng = math.split_off(x);
    eng.sort();
    eng.sort_by_key(|&i| Reverse(b[i]));
    let mut math_eng = eng.split_off(y);
    math_eng.sort();
    math_eng.sort_by_key(|&i| Reverse(a[i] + b[i]));
    let _rest = math_eng.split_off(z);
    let mut ans = Vec::new();
    ans.extend(math);
    ans.extend(eng);
    ans.extend(math_eng);
    ans.sort();
    for ans in ans {
        println!("{}", ans + 1);
    }
}
