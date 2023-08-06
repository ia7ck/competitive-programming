use std::collections::{HashMap, HashSet};

use proconio::input;

fn main() {
    input! {
        r: usize,
        c: usize,
        r_s: usize,
        c_s: usize,
        r_t: usize,
        c_t: usize,
    };

    let mut nexts = HashSet::new();
    let mut count_y = vec![0; r + 1];
    let mut count_x = vec![0; c + 1];
    let mut count_diag_1 = HashMap::new();
    let mut count_diag_2 = HashMap::new();
    macro_rules! update {
        ($y: expr, $x: expr) => {
            nexts.insert(($y, $x));
            count_y[$y] += 1;
            count_x[$x] += 1;
            *count_diag_1.entry($y as isize - $x as isize).or_insert(0) += 1;
            *count_diag_2.entry($y as isize + $x as isize).or_insert(0) += 1;
        };
    }

    // (r_t, c_t) からの一手
    for y in 1..=r {
        if y != r_t {
            update!(y, c_t);
        }
    }
    for x in 1..=c {
        if x != c_t {
            update!(r_t, x);
        }
    }
    for (y, x) in (1..r_t).rev().zip((1..c_t).rev()) {
        update!(y, x);
    }
    for (y, x) in ((r_t + 1)..=r).zip((c_t + 1)..=c) {
        update!(y, x);
    }
    for (y, x) in (1..r_t).rev().zip((c_t + 1)..=c) {
        update!(y, x);
    }
    for (y, x) in ((r_t + 1)..=r).zip((1..c_t).rev()) {
        update!(y, x);
    }

    // eprintln!("{:?}", count_y);
    // eprintln!("{:?}", count_x);
    // eprintln!("{:?}", count_diag_1);
    // eprintln!("{:?}", count_diag_2);

    let mut ans = 0_u64;
    macro_rules! add {
        ($y: expr, $x: expr) => {
            // eprintln!("add ({}, {}) y = {}, x = {}, diag_1 = {}, diag_2 = {}, nexts = {}", $y, $x, count_y[$y], count_x[$x], count_diag_1.get(&($y as isize - $x as isize)).unwrap_or(&0), count_diag_2.get(&($x as isize - $y as isize)).unwrap_or(&0), nexts.contains(&($y, $x)));
            ans += count_y[$y];
            ans += count_x[$x];
            ans += count_diag_1.get(&($y as isize - $x as isize)).unwrap_or(&0);
            ans += count_diag_2.get(&($y as isize + $x as isize)).unwrap_or(&0);
            if nexts.contains(&($y, $x)) {
                ans -= 4;
            }
        };
    }

    // (r_s, c_s) からの一手
    for y in 1..=r {
        if y != r_s {
            add!(y, c_s);
        }
    }
    for x in 1..=c {
        if x != c_s {
            add!(r_s, x);
        }
    }
    for (y, x) in (1..r_s).rev().zip((1..c_s).rev()) {
        add!(y, x);
    }
    for (y, x) in ((r_s + 1)..=r).zip((c_s + 1)..=c) {
        add!(y, x);
    }
    for (y, x) in (1..r_s).rev().zip((c_s + 1)..=c) {
        add!(y, x);
    }
    for (y, x) in ((r_s + 1)..=r).zip((1..c_s).rev()) {
        add!(y, x);
    }
    println!("{}", ans);
}
