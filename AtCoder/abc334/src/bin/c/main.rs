use proconio::input;

fn main() {
    input! {
        _n: usize,
        k: usize,
        a: [usize; k],
    };

    if k % 2 == 0 {
        let mut ans = 0;
        for i in (1..k).step_by(2) {
            ans += a[i] - a[i - 1];
        }
        println!("{}", ans);
        return;
    }

    let mut front = vec![0; k];
    for i in 1..k {
        front[i] = front[i - 1] + (a[i] - a[i - 1]);
    }
    let mut back = vec![0; k];
    for i in (0..k - 1).rev() {
        back[i] = back[i + 1] + (a[i + 1] - a[i]);
    }
    let mut ans = usize::MAX;
    for i in (0..k).step_by(2) {
        let mut tmp = 0;
        if i >= 1 {
            tmp += front[i - 1];
        }
        if i + 1 < k {
            tmp += back[i + 1];
        }
        ans = ans.min(tmp);
    }
    println!("{}", ans);
}

// 3 3
// 1 10 100
