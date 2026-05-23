use proconio::{input, marker::Usize1};

fn main() {
    input! {
        x: Usize1,
    };

    let mut ans = "HelloWorld".chars().collect::<Vec<_>>();
    ans.remove(x);

    println!("{}", ans.iter().collect::<String>());
}
