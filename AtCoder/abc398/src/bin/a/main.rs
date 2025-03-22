use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut ans = String::new();
    if n % 2 == 0 {
        for _ in 0..(n / 2 - 1) {
            ans.push('-');
        }
        ans.push_str("==");
        for _ in 0..(n / 2 - 1) {
            ans.push('-');
        }
    } else {
        for _ in 0..(n / 2) {
            ans.push('-');
        }
        ans.push_str("=");
        for _ in 0..(n / 2) {
            ans.push('-');
        }
    }

    println!("{}", ans);
}
