use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [isize; n],
    };

    let mut cum_sum = vec![0; n];
    for i in 0..n {
        if i >= 1 {
            cum_sum[i] += cum_sum[i - 1];
        }
        assert!(cum_sum[i] >= 0);
        a[i] += cum_sum[i];
        if a[i] >= 1 && i + 1 < n {
            cum_sum[i + 1] += 1;
            if i + (a[i] as usize) + 1 < n {
                cum_sum[i + (a[i] as usize) + 1] -= 1;
            }
            a[i] -= a[i].min((n - i - 1) as isize);
        }
    }

    println!(
        "{}",
        a.iter()
            .map(|&x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
