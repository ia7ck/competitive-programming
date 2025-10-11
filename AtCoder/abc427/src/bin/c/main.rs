use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    };

    let mut ans = m;
    for bits in 0..(1 << n) {
        let left = (0..n).filter(|&i| bits >> i & 1 == 0).collect::<Vec<_>>();
        let right = (0..n).filter(|&i| bits >> i & 1 == 1).collect::<Vec<_>>();

        let mut rm = 0;
        rm += edges
            .iter()
            .filter(|&&(u, v)| left.contains(&u) && left.contains(&v))
            .count();
        rm += edges
            .iter()
            .filter(|&&(u, v)| right.contains(&u) && right.contains(&v))
            .count();
        ans = ans.min(rm);
    }

    println!("{}", ans);
}
