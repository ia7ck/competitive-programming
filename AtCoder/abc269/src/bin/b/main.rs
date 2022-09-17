use proconio::input;

fn generate(a: usize, b: usize, c: usize, d: usize) -> Vec<String> {
    let mut result = vec![vec!['.'; 10]; 10];
    for i in (a - 1)..b {
        for j in (c - 1)..d {
            result[i][j] = '#';
        }
    }
    result
        .into_iter()
        .map(|row| row.into_iter().collect())
        .collect()
}

fn main() {
    input! {
        ss: [String; 10],
    };

    for a in 1..=10 {
        for b in a..=10 {
            for c in 1..=10 {
                for d in c..=10 {
                    let tt = generate(a, b, c, d);
                    if ss == tt {
                        println!("{} {}",  a, b);
                        println!("{} {}",  c, d);
                        return;
                    }
                }
            }
        }
    }

    unreachable!();
}
