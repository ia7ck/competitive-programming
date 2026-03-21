use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut cost = vec![vec![]; n - 1];
    for i in 0..(n - 1) {
        input! {
            row: [u64; n - i - 1],
        };
        cost[i] = row;
    }

    for a in 1..n {
        for b in (a + 1)..=n {
            for c in (b + 1)..=n {
                if cost[a - 1][b - a - 1] + cost[b - 1][c - b - 1] < cost[a - 1][c - a - 1] {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}
