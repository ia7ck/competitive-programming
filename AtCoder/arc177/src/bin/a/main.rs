use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        a_1: usize,
        a_5: usize,
        a_10: usize,
        a_50: usize,
        a_100: usize,
        a_500: usize,
        n: usize,
        xs: [usize; n],
    };

    // 500, 100, 50, 10, 5, 1 の順に使えばよさそう

    let mut memo = HashMap::new();
    let ans = solve(0, &xs, a_1, a_5, a_10, a_50, a_100, a_500, &mut memo);
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn solve(
    i: usize,
    xs: &Vec<usize>,
    a_1: usize,
    a_5: usize,
    a_10: usize,
    a_50: usize,
    a_100: usize,
    a_500: usize,
    // memo いる？
    memo: &mut HashMap<(usize, usize, usize, usize, usize, usize, usize), bool>,
) -> bool {
    if i == xs.len() {
        return true;
    }

    if let Some(res) = memo.get(&(i, a_1, a_5, a_10, a_50, a_100, a_500)) {
        return *res;
    }

    let mut x = xs[i];
    let c_500 = (x / 500).min(a_500);
    x -= c_500 * 500;
    let c_100 = (x / 100).min(a_100);
    x -= c_100 * 100;
    let c_50 = (x / 50).min(a_50);
    x -= c_50 * 50;
    let c_10 = (x / 10).min(a_10);
    x -= c_10 * 10;
    let c_5 = (x / 5).min(a_5);
    x -= c_5 * 5;
    let c_1 = x.min(a_1);
    x -= c_1 * 1;

    let res = if x == 0 {
        solve(
            i + 1,
            xs,
            a_1 - c_1,
            a_5 - c_5,
            a_10 - c_10,
            a_50 - c_50,
            a_100 - c_100,
            a_500 - c_500,
            memo,
        )
    } else {
        false
    };

    memo.insert((i, a_1, a_5, a_10, a_50, a_100, a_500), res);
    res
}
