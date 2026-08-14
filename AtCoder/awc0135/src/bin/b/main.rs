use proconio::input;

fn main() {
    input! {
        n: usize,
        ps: [(u64, u64); n],
    };

    let mut ord = (0..n).collect::<Vec<_>>();
    ord.sort_by(|&i, &j| {
        let (pi, si) = ps[i];
        let (pj, sj) = ps[j];

        // si/pi, sj/pj
        (si * pj).cmp(&(sj * pi)).reverse()
    });

    let ans = ord[0] + 1;
    println!("{}", ans);
}
