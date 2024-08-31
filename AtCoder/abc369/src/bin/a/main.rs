use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    };

    let mut ans = 0;
    for x in -200..=200 {
        let mut v = vec![a, b, x];
        v.sort();
        if v[1] - v[0] == v[2] - v[1] {
            ans += 1;
        }
    }
    println!("{}", ans);
}
