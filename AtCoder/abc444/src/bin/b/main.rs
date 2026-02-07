use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };

    let mut ans = 0;
    for i in 1..=n {
        let sum = digit_sum(i);
        if sum == k {
            ans += 1;
        }
    }

    println!("{}", ans);
}

fn digit_sum(n: usize) -> usize {
    let mut sum = 0;
    let mut n = n;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}
