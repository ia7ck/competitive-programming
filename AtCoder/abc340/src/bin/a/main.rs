use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        d: i32,
    };

    assert_eq!((b - a) % d, 0);
    let n = (b - a) / d;

    let mut ans = Vec::new();
    for i in 0..=n {
        ans.push(a + i * d);
    }
    assert_eq!(ans.last(), Some(&b));
    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
