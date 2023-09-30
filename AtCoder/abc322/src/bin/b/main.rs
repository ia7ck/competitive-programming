use proconio::input;

fn main() {
    input! {
        _n: usize,
        _m: usize,
        s: String,
        t: String,
    };

    let ans = match (t.starts_with(&s), t.ends_with(&s)) {
        (true, true) => 0,
        (true, false) => 1,
        (false, true) => 2,
        (false, false) => 3,
    };

    println!("{}", ans);
}
