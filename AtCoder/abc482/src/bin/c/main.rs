use proconio::input;

fn main() {
    input! {
        _h: usize,
        _w: usize,
        n: usize,
        q: usize,
        rc: [(usize, usize); n],
    };

    let mut oy = rc.clone();
    oy.sort_unstable();
    let mut ox = rc.clone();
    ox.sort_unstable_by_key(|&(_, c)| c);

    let count_d = |d: usize| -> usize { oy.partition_point(|&(x, _)| x <= d) };
    let count_r = |r: usize| -> usize { ox.partition_point(|&(_, y)| y <= r) };
    for _ in 0..q {
        input! {
            op: u8,
        };
        if op == 1 {
            input! {
                u: usize,
                d: usize,
            };

            let ans = count_d(d) - count_d(u - 1);
            println!("{}", ans);
        } else {
            input! {
                l: usize,
                r: usize,
            };

            let ans = count_r(r) - count_r(l - 1);
            println!("{}", ans);
        }
    }
}
