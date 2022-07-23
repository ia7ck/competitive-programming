use proconio::input;

fn main() {
    input! {
        l: u8,
        r: u8,
        s: u8,
        t: u8,
    };

    let ((_l, r), (s, t)) = if (l, r) <= (s, t) {
        ((l, r), (s, t))
    } else {
        ((s, t), (l, r))
    };

    let ans = r.saturating_sub(s).min(t - s);
    println!("{}", ans);
}
