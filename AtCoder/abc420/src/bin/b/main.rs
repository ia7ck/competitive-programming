use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    };

    let mut score = vec![0; n];
    for j in 0..m {
        let zero = (0..n).filter(|&i| s[i][j] == '0').collect::<Vec<_>>();
        let one = (0..n).filter(|&i| s[i][j] == '1').collect::<Vec<_>>();
        if zero.is_empty() || one.is_empty() {
            for i in 0..n {
                score[i] += 1;
            }
        } else if zero.len() < one.len() {
            for i in zero {
                score[i] += 1;
            }
        } else {
            assert!(zero.len() > one.len());
            for i in one {
                score[i] += 1;
            }
        }
    }

    let max_score = score.iter().copied().max().unwrap();
    let ans = (0..n)
        .filter(|&i| score[i] == max_score)
        .collect::<Vec<_>>();

    println!(
        "{}",
        ans.iter()
            .map(|x| (x + 1).to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
