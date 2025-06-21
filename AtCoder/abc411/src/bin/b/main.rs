use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [u32; n - 1],
    };

    for i in 0..(n - 1) {
        let ans = d[i..]
            .iter()
            .scan(0, |acc, &x| {
                *acc += x;
                Some(*acc)
            })
            .collect::<Vec<_>>();
        println!(
            "{}",
            ans.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}
