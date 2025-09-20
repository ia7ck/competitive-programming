use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        ab: [(Usize1, Usize1); k],
    };

    let mut score = vec![0; n];
    let mut ans = Vec::new();
    for (a, _) in ab {
        score[a] += 1;
        if score[a] == m {
            ans.push(a);
        }
    }

    if ans.is_empty() {
        return;
    }
    println!(
        "{}",
        ans.iter()
            .map(|x| (x + 1).to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
