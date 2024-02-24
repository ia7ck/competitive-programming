use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut ans = String::new();
    for _ in 0..n {
        ans.push('1');
        ans.push('0');
    }
    ans.push('1');
    println!("{}", ans);
}
