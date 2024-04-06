use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut ans = String::new();
    for i in 1..=n {
        if i % 3 == 0 {
            ans.push('x');
        } else {
            ans.push('o');
        }
    }
    println!("{}", ans);
}
