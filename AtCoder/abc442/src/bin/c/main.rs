use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    };

    let mut deg = vec![0; n];
    for (a, b) in ab {
        deg[a] += 1;
        deg[b] += 1;
    }

    let mut ans = Vec::new();
    for i in 0..n {
        let c = n - deg[i] - 1;
        let res = if c < 3 {
            0
        } else {
            c * (c - 1) * (c - 2) / 3 / 2 / 1
        };
        ans.push(res);
    }

    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
