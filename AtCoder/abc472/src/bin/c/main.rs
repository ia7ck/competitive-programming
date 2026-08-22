use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: u64,
        a: [u64; n],
    };

    let mut ans = Vec::<bool>::new();
    let mut sum = 0;
    for i in 0..n {
        if i >= m && ans[i - m] {
            sum -= a[i - m];
        }

        let eat = sum + a[i] <= k;
        ans.push(eat);

        if eat {
            sum += a[i];
        }
    }

    for ans in ans {
        if ans {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
