use proconio::input;

fn main() {
    input! {
        s: [String; 12],
    };

    let mut ans = 0;
    for i in 0..12 {
        if s[i].len() == i + 1 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
