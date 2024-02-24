use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
        q: usize,
        ab: [(Usize1, Usize1); q],
    };

    for (a, b) in ab {
        let p_a = p.iter().position(|&x| x == a).unwrap();
        let p_b = p.iter().position(|&x| x == b).unwrap();
        if p_a < p_b {
            println!("{}", a + 1);
        } else if p_a > p_b {
            println!("{}", b + 1);
        } else {
            unreachable!();
        }
    }
}
