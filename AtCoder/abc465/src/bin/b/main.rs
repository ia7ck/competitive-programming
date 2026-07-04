use proconio::input;

fn main() {
    input! {
        x: i32,
        y: i32,
        l: i32,
        r: i32,
        a: i32,
        b: i32,
    };

    let mut ans = (b - a) * y;
    let la = l.max(a);
    let rb = r.min(b);
    if la < rb {
        ans -= (rb - la) * y;
        ans += (rb - la) * x;
    }

    println!("{}", ans);
}
