use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
        b: [u64; n],
    };

    if a == b {
        println!("No");
        return;
    }

    let mut gain = 0;
    let mut loss = 0;
    for i in 0..n {
        if a[i] > b[i] {
            gain += a[i] - b[i];
        }
        if a[i] < b[i] {
            loss += b[i] - a[i];
        }
    }
    let w_max = 10_u64.pow(18);
    if gain.saturating_mul(w_max) > loss * 1 {
        let w = (0..n)
            .map(|i| if a[i] > b[i] { w_max } else { 1 })
            .collect::<Vec<_>>();
        println!("Yes");
        println!(
            "{}",
            w.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    } else {
        println!("No");
    }
}
