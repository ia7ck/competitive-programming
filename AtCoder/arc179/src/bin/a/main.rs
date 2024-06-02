use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        mut a: [i64; n],
    };

    let ans = if k > 0 {
        a.sort();
        let mut cum_sum = vec![0; n + 1];
        for i in 0..n {
            cum_sum[i + 1] = cum_sum[i] + a[i];
        }

        let i = cum_sum.iter().position(|&x| x >= k).unwrap_or(n + 1);
        let (_left, right) = cum_sum.split_at(i);
        if right.iter().all(|&x| x >= k) {
            Some(a)
        } else {
            None
        }
    } else {
        a.sort();
        a.reverse();
        let mut cum_sum = vec![0; n + 1];
        for i in 0..n {
            cum_sum[i + 1] = cum_sum[i] + a[i];
        }
        if cum_sum.iter().all(|&x| x >= k) {
            Some(a)
        } else {
            None
        }
    };

    if let Some(ans) = ans {
        println!("Yes");
        let ans = ans
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        println!("{}", ans);
    } else {
        println!("No");
    }
}
