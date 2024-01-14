use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut ans = String::new();
    ans.push('L');
    for _ in 0..n {
        ans.push('o');
    }
    ans.push('n');
    ans.push('g');
    println!("{}", ans);
}
