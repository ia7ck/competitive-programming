use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        x: [usize; q],
    };

    let mut ans = Vec::new();
    let mut b = vec![0; n + 1];
    for x in x {
        if x >= 1 {
            b[x] += 1;
            ans.push(x);
        } else {
            let min = b[1..].iter().min().copied().unwrap();
            let pos = (1..=n).find(|&i| b[i] == min).unwrap();
            b[pos] += 1;
            ans.push(pos);
        }
    }

    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
