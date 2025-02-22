use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ss: [String; n],
    };

    ss.sort_by_key(|s| s.len());
    for s in ss {
        print!("{}", s);
    }
    println!();
}
