use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        cs: [(usize, u32); n],
    };

    let mut s_by_c = vec![vec![]; m + 1];
    for (c, s) in cs {
        s_by_c[c].push(s);
    }

    for k in 1..=m {
        if let Some(max) = s_by_c[k].iter().max() {
            print!("{}", max);
        } else {
            print!("-1");
        }

        if k < m {
            print!(" ");
        }
    }
}
