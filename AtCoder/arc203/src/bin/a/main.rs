use proconio::input;

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
        };

        solve(n, m);
    }
}

fn solve(n: usize, m: usize) {
    if m % 2 == 0 {
        let ans = n * (m / 2);
        println!("{}", ans);
    } else {
        let ans = (m + 1) / 2 + (n - 1) * (m / 2) ;
        println!("{}", ans);
    }
}
