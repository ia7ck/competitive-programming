use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut count = vec![0; n + 1];
    let mut ans = 1_usize;
    for i in 0..n {
        if i >= 1 && a[i - 1] != a[i] {
            ans += i - count[a[i]];
        }
        count[a[i]] += 1;
    }

    println!("{}", ans);
}

// 1 4 1 4 2 1
//
// 1 1 4 1 4 2
// 1 4 1 1 4 2
// 1 4 1 4 1 2
//
// 2 1 4 1 4 1
// 1 2 4 1 4 1
// 1 4 2 1 4 1
// 1 4 1 2 4 1
//
// 4 1 4 1 2 1
// 1 4 4 1 2 1
//
// 1 1 4 4 2 1
//
// 4 1 1 4 2 1
