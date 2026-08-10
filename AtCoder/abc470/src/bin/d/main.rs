use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        mut p: [Usize1; n],
    };

    let mut inv_p = vec![0; n];
    for i in 0..n {
        inv_p[p[i]] = i;
    }

    for _ in 0..q {
        input! {
            op: u8,
        };

        if op == 1 {
            input! {
                x: Usize1,
                y: Usize1,
            };

            inv_p.swap(p[x], p[y]);
            p.swap(x, y);
        } else {
            (p, inv_p) = (inv_p, p);
        }
    }

    println!(
        "{}",
        p.iter()
            .map(|x| (x + 1).to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
