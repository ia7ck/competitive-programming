use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        lr: [(Usize1, Usize1); m],
    };

    let mut imos = vec![0_isize; n + 1];
    for (l, r) in lr {
        imos[l] += 1;
        imos[r + 1] -= 1;
    }
    for i in 0..n {
        imos[i + 1] += imos[i];
    }
    assert_eq!(imos[n], 0);

    let ans = imos[..n].iter().min().copied().unwrap();
    println!("{}", ans);
}
