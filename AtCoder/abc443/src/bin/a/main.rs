use proconio::input;

fn main() {
    input! {
        s: String,
    };

    let mut ans = s;
    ans.push('s');

    println!("{}", ans);
}
